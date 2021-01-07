import os
import logging
import logging.config

import yaml

LOGGING_SETTINGS = {}

try:
    with open('py-logging.yml', 'r') as f:
        LOGGING_SETTINGS = yaml.load(f, yaml.CLoader)
except FileNotFoundError:
    pass

logging.config.dictConfig(LOGGING_SETTINGS)

logging.getLogger(__name__).debug('Trying logs')
