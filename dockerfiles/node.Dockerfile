FROM messense/badwolf-test-runner:base
MAINTAINER Messense Lv <messense@icloud.com>

ENV NPM_CONFIG_LOGLEVEL warn

RUN curl -sL https://deb.nodesource.com/setup_6.x | bash - \
    && apt-get install -y nodejs \
    && rm -rf /tmp/* /var/tmp/*
