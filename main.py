#!/usr/bin/env python

import logging
import os
import argparse
from lib import load_vector_config, vector_config_to_dot

logging.basicConfig(level=os.getenv('LOG_LEVEL', 'WARN'))
logger = logging.getLogger(__name__)

parser = argparse.ArgumentParser()
parser.add_argument('config_file_path', nargs='+')
args = parser.parse_args()
config_file_paths = args.config_file_path

config = load_vector_config(config_file_paths)
dot_body = vector_config_to_dot(config)

print(dot_body)
