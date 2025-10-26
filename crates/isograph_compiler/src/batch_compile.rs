use std::{path::PathBuf, time::Duration};

use crate::{
    SourceError,
    compiler_state::CompilerState,
    with_duration::WithDuration,
    write_artifacts::{GenerateArtifactsError, write_artifacts_to_disk},
};
use colored::Colorize;
use common_lang_types::CurrentWorkingDirectory;
use generate_artifacts::{
    generate_artifacts::GetArtifactPathAndContentError, get_artifact_path_and_content,
};
use isograph_schema::{IsographDatabase, NetworkProtocol};
use pretty_duration::pretty_duration;
use thiserror::Error;
use tracing::{error, info};

pub struct CompilationStats {
    pub client_field_count: usize,
    pub client_pointer_count: usize,
    pub entrypoint_count: usize,
    pub total_artifacts_written: usize,
}

pub fn compile_and_print<TNetworkProtocol: NetworkProtocol + 'static>(
    config_location: &PathBuf,
    current_working_directory: CurrentWorkingDirectory,
) -> Result<(), BatchCompileError<TNetworkProtocol>> {
    info!("{}", "Starting to compile.".cyan());
    let state = CompilerState::new(config_location, current_working_directory)?;
    print_result(WithDuration::new(|| compile::<TNetworkProtocol>(&state.db)))
}

pub fn print_result<TNetworkProtocol: NetworkProtocol + 'static>(
    result: WithDuration<Result<CompilationStats, BatchCompileError<TNetworkProtocol>>>,
) -> Result<(), BatchCompileError<TNetworkProtocol>> {
    match result.item {
        Ok(stats) => {
            print_stats(result.elapsed_time, stats);
            Ok(())
        }
        Err(err) => {
            error!(
                "{}\n{}\n{}",
                "Error when compiling.\n".bright_red(),
                err,
                format!(
                    "Compilation took {}.",
                    pretty_duration(&result.elapsed_time, None)
                )
                .bright_red()
            );
            Err(err)
        }
    }
}

fn print_stats(elapsed_time: Duration, stats: CompilationStats) {
    let s_if_plural = |count: usize| {
        if count == 1 { "" } else { "s" }
    };

    info!(
        "Successfully compiled {} client field{}, {} client pointer{}, {} \
        entrypoint{}, and wrote {} artifact{}, in {}.",
        stats.client_field_count,
        s_if_plural(stats.client_field_count),
        stats.client_pointer_count,
        s_if_plural(stats.client_pointer_count),
        stats.entrypoint_count,
        s_if_plural(stats.entrypoint_count),
        stats.total_artifacts_written,
        s_if_plural(stats.total_artifacts_written),
        pretty_duration(&elapsed_time, None)
    );
}

/// This the "workhorse" command of batch compilation.
#[tracing::instrument(skip(db))]
pub fn compile<TNetworkProtocol: NetworkProtocol + 'static>(
    db: &IsographDatabase<TNetworkProtocol>,
) -> Result<CompilationStats, BatchCompileError<TNetworkProtocol>> {
    // Note: we calculate all of the artifact paths and contents first, so that writing to
    // disk can be as fast as possible and we minimize the chance that changes to the file
    // system occur while we're writing and we get unpredictable results.

    let config = db.get_isograph_config();
    let (artifacts, stats) = get_artifact_path_and_content(db)?;
    let total_artifacts_written =
        write_artifacts_to_disk(artifacts, &config.artifact_directory.absolute_path)?;

    Ok(CompilationStats {
        client_field_count: stats.client_field_count,
        client_pointer_count: stats.client_pointer_count,
        entrypoint_count: stats.entrypoint_count,
        total_artifacts_written,
    })
}

#[derive(Error, Debug)]
pub enum BatchCompileError<TNetworkProtocol: NetworkProtocol + 'static> {
    #[error("{error}")]
    SourceError {
        #[from]
        error: SourceError,
    },

    #[error("{error}")]
    GenerateArtifacts {
        #[from]
        error: GenerateArtifactsError,
    },

    #[error(
        "{}",
        messages.iter().fold(String::new(), |mut output, x| {
            output.push_str(&format!("\n\n{x}"));
            output
        })
    )]
    NotifyErrors { messages: Vec<notify::Error> },

    #[error("{error}")]
    GenerateArtifactsError {
        #[from]
        error: GetArtifactPathAndContentError<TNetworkProtocol>,
    },
}

impl<TNetworkProtocol: NetworkProtocol + 'static> From<Vec<notify::Error>>
    for BatchCompileError<TNetworkProtocol>
{
    fn from(messages: Vec<notify::Error>) -> Self {
        BatchCompileError::NotifyErrors { messages }
    }
}
