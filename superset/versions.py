versions = [
    {
        "product": "4.0.2",
        "python": "3.9",
        "vector": "0.43.1",
        "statsd_exporter": "0.28.0",
        "authlib": "1.2.1",  # https://github.com/dpgaspar/Flask-AppBuilder/blob/release/4.4.1/requirements/extra.txt#L7
        "stackable-base": "1.0.0",
    },
    {
        "product": "4.1.1",
        "python": "3.9",  # 3.11 support was merged in January 2025 (two months after 4.1.1 release), 3.10 is not available in our UBI image, so we need to stay on 3.9 for now
        "vector": "0.43.1",
        "statsd_exporter": "0.28.0",
        "authlib": "1.2.1",  # https://github.com/dpgaspar/Flask-AppBuilder/blob/release/4.5.0/requirements/extra.txt#L7
        "stackable-base": "1.0.0",
    },
]
