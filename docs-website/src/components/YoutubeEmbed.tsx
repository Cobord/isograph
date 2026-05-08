export default function YoutubeEmbed() {
  return (
    <section>
      <div className="container padding-bottom--xl padding-top--xl">
        <div className="row">
          <div className="col col--8 col--offset-2">
            <h2 className="text--center">
              Watch the talk at GraphQL&nbsp;Conf 2025
            </h2>
          </div>
        </div>
        <div className="row">
          <div className="col col--8 col--offset-2 margin-top--md">
            <iframe
              width="100%"
              height="444"
              src="https://www.youtube-nocookie.com/embed/3GWZ9yiskFk"
              title="GraphQL in a World of Full-stack, Rich Clients"
              allow="autoplay; clipboard-write; encrypted-media; picture-in-picture; web-share"
              allowFullScreen
              frameBorder="0"
            ></iframe>
          </div>
        </div>
      </div>
    </section>
  );
}
