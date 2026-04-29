from setuptools import setup, find_packages

setup(
    name="olaforge",
    version="2.0.0",
    description="OlaForge Python SDK - AI Agent 安全沙箱执行引擎",
    author="OlaForge Team",
    author_email="team@olaforge.dev",
    url="https://github.com/zyzheal/OlaForge",
    packages=find_packages(),
    install_requires=[],
    extras_require={
        "dev": ["pytest", "black", "mypy"],
    },
    python_requires=">=3.7",
    entry_points={
        "console_scripts": [
            "olaforge-shell=olaforge.cli:main",
        ],
    },
    classifiers=[
        "Development Status :: 4 - Beta",
        "Intended Audience :: Developers",
        "License :: OSI Approved :: MIT License",
        "Programming Language :: Python :: 3",
        "Programming Language :: Python :: 3.7",
        "Programming Language :: Python :: 3.8",
        "Programming Language :: Python :: 3.9",
        "Programming Language :: Python :: 3.10",
        "Programming Language :: Python :: 3.11",
        "Topic :: Software Development :: Libraries :: Python Modules",
        "Topic :: Security",
    ],
    keywords="sandbox security ai agent execution",
)