FROM messense/badwolf-test-runner:base
MAINTAINER Messense Lv <messense@icloud.com>

RUN apt-get update && \
    apt-get install -y --no-install-recommends software-properties-common && \
    add-apt-repository -y ppa:fkrull/deadsnakes && \
    add-apt-repository -y ppa:fkrull/deadsnakes-python2.7 && \
    add-apt-repository -y ppa:pypy/ppa && \
    wget https://bitbucket.org/pypa/setuptools/raw/bootstrap/ez_setup.py -O /tmp/ez_setup.py && \
    apt-get update && \
    DEBIAN_FRONTEND=noninteractive apt-get install -y --no-install-recommends \
    python2.6 python2.6-dev \
    python2.7 python2.7-dev \
    python3.4 python3.4-dev \
    python3.5 python3.5-dev \
    pypy && \
    ln -s /usr/bin/python2.7 /usr/bin/python && \
    ln -s /usr/bin/python2.7 /usr/bin/python2 && \
    python2.6 /tmp/ez_setup.py && easy_install-2.6 pip && \
    python3.4 /tmp/ez_setup.py && easy_install-3.4 pip && \
    python3.5 /tmp/ez_setup.py && easy_install-3.5 pip && \
    python2.7 /tmp/ez_setup.py && easy_install-2.7 pip && \
    DEBIAN_FRONTEND=noninteractive apt-get autoremove -y && \
    rm -rf /var/lib/apt/lists/* \
    /tmp/* \
    /var/tmp/*
