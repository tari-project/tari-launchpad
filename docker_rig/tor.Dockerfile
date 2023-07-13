# Package build

ARG ALPINE_VERSION=3.18

FROM alpine:$ALPINE_VERSION

ARG ALPINE_VERSION
ARG BUILDPLATFORM
ARG VERSION=1.0.1

# https://pkgs.alpinelinux.org/packages?name=tor&branch=v3.18&repo=community&arch=&maintainer=
ARG TOR_VERSION=0.4.7.13-r2

# Install tor with a minimum version
RUN apk add --no-cache grep curl tor>$TOR_VERSION

ENV dockerfile_version=$VERSION
ENV dockerfile_build_arch=$BUILDPLATFORM
ENV alpine_version=$ALPINE_VERSION
ENV tor_version=$TOR_VERSION

# SocksPort
EXPOSE 9050
# ControlPort
EXPOSE 9051

VOLUME ["/etc/tor", "/var/lib/tor"]

# Tell Docker to periodically run curl as a way of checking that Tor is runnning OK,
# and is able to build a circuit. Link goes to a Tor Project page, which checks that
# client is accessing it through Tor network and not directly. It gives false negatives
# sometimes, so we should allow several retries.
#
# --socks5-hostname parameter is very important - it tells curl to ask proxy (Tor) for DNS lookup,
#   instead of doing it on its own - the behavior that torrc file above explicitly prohibits,
#   because it opens a possibility for a traffic correlation attack.
#
# --location flag is added just in case Tor Project changes the location of the page and puts a redirect at
#   the previos location, so curl can follow that redirect.
#
# grep gets the output of curl and looks for first occurence of the string 'Congratulations',
# exits with 0 if found and 1 otherwise. Nothing is printed to stdout during this command.

# Currently this healthcheck is problematic and may need very long timeout periods to operate. The endpoint we're using
# isn't controlled by us and sometimes takes up to 45seconds to return, othertimes it gets denied by cloudflare.
# Additionally the HEALTHCHECK emits docker events that launchpad doesn't currently handle well, and will cause the
# container to show as offline.
# HEALTHCHECK --interval=120s --timeout=30s --start-period=60s --retries=5 \
#             CMD curl --silent --location --socks5-hostname localhost:9050 https://check.torproject.org/?lang=en_US | \
#             grep -qm1 Congratulations

USER tor
ENTRYPOINT ["/usr/bin/tor"]
CMD ["-f", "/etc/tor/torrc", "--allow-missing-torrc"]
