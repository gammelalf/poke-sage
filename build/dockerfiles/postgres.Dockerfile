FROM postgres:17-bookworm AS build

RUN <<EOF
set -e
apt-get update
apt-get install -y python3 python3-venv python3-pip git
EOF

RUN <<EOF
set -e
git clone --depth 1 --recurse-submodules --shallow-submodules https://github.com/PokeAPI/pokeapi.git
EOF

WORKDIR /pokeapi

RUN <<EOF
set -e
python3 -m venv venv
venv/bin/pip install -r requirements.txt
EOF

COPY ./build/pokeapi/settings.py ./config/poke_sage.py

RUN <<EOF
set -e
su postgres -c 'initdb'
su postgres -c 'pg_ctl -D /var/lib/postgresql/data start'
su postgres -c 'createdb pokeapi'
venv/bin/python manage.py migrate --settings=config.poke_sage pokemon_v2
venv/bin/python manage.py shell --settings=config.poke_sage -c "from data.v2.build import build_all; build_all()"
su postgres -c 'pg_dump pokeapi > /var/lib/postgresql/dump.sql'
su postgres -c 'pg_ctl -D /var/lib/postgresql/data stop'
EOF

FROM postgres:17-alpine AS final

COPY --from=build /var/lib/postgresql/dump.sql /docker-entrypoint-initdb.d/pokeapi_dump.sql