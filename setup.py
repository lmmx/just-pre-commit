from setuptools import setup, find_packages
import os
import glob

# Include the vendored wheel file in package_data
package_data = {
    'just_pre_commit.vendor': ['*.whl']
}

setup(
    name="just-pre-commit",
    version="0.1.0",
    packages=find_packages(),
    package_data=package_data,
    entry_points={
        'console_scripts': [
            'just-ensure=just_pre_commit.install:main',
        ],
    },
    python_requires=">=3.6",
)