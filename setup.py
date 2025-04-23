from setuptools import setup, find_packages

setup(
    name='just-pre-commit',
    version='0.1.0',
    packages=find_packages(),
    install_requires=[
        'rust-just',  # This will install the just wheel from PyPI
    ],
    entry_points={
        'console_scripts': [
            'ensure-just=just_pre_commit.ensure_just:main',
        ],
    },
)