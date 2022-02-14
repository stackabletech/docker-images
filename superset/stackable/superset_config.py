import os
from superset.stats_logger import StatsdStatsLogger

SECRET_KEY = os.environ.get("SECRET_KEY")
SQLALCHEMY_DATABASE_URI = os.environ.get("SQLALCHEMY_DATABASE_URI")
STATS_LOGGER = StatsdStatsLogger(host='0.0.0.0', port=9125)
