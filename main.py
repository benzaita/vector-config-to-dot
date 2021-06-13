#!/usr/bin/env python

import logging
import os
import sys
import toml
from lib import vector_config_to_dot

logging.basicConfig(level=os.getenv('LOG_LEVEL', 'WARN'))
logger = logging.getLogger(__name__)

config = toml.load(sys.stdin)
dot_body = vector_config_to_dot(config)

print(dot_body)
