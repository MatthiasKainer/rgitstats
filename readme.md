# rgitstats

An application that generates simple stats from your conventional commits

## Install

```bash
RELEASE_VERSION=0.1.3
RELEASE_OS=x86_64-unknown-linux-gnu # or x86_64-apple-darwin
curl -L -o rgitstats https://github.com/MatthiasKainer/rgitstats/releases/download/$RELEASE_VERSION/rgitstats-$RELEASE_VERSION-$RELEASE_OS
chmod +x rgitstats
./rgitstats path/to/my/repo
```

You can also move it to you bin path from there to have it globally available

## Usage

Usage: rgitstats [OPTIONS] \<PATH>...

### Arguments:
  \<PATH>...  Git repo(s) to check

### Options:
* --result <RESULT>  [default: types] [possible values: types, scope, authors, every]
* -h, --help             Print help

```bash
rgiststats path/to/my/repo

+-------+-------+------------+
| Type  | Count | Percentage |
+=======+=======+============+
| feat  | 4     | 36.36%     |
+-------+-------+------------+
| build | 4     | 36.36%     |
+-------+-------+------------+
| ci    | 2     | 18.18%     |
+-------+-------+------------+
| docs  | 1     | 9.09%      |
+-------+-------+------------+
```

```bash
❯ rgitstats --result authors .
+-----------------+-------+------------+
| Type            | Count | Percentage |
+=================+=======+============+
| Matthias Kainer | 11    | 100.00%    |
+-----------------+-------+------------+
```

```bash

❯ rgitstats --result scope .
+---------+-------+------------+--------------------------------------------------------------------------+
| Scope   | Count | Percentage | Type                                                                     |
+=========+=======+============+==========================================================================+
| cli     | 2     | 50.00%     | +------+-------+------------+------------------------------------------+ |
|         |       |            | | Type | Count | Percentage | Author                                   | |
|         |       |            | +======+=======+============+==========================================+ |
|         |       |            | | feat | 2     | 100.00%    | +-----------------+-------+------------+ | |
|         |       |            | |      |       |            | | Author          | Count | Percentage | | |
|         |       |            | |      |       |            | +=================+=======+============+ | |
|         |       |            | |      |       |            | | Matthias Kainer | 2     |            | | |
|         |       |            | |      |       |            | +-----------------+-------+------------+ | |
|         |       |            | +------+-------+------------+------------------------------------------+ |
+---------+-------+------------+--------------------------------------------------------------------------+
| types   | 1     | 25.00%     | +------+-------+------------+------------------------------------------+ |
|         |       |            | | Type | Count | Percentage | Author                                   | |
|         |       |            | +======+=======+============+==========================================+ |
|         |       |            | | feat | 1     | 100.00%    | +-----------------+-------+------------+ | |
|         |       |            | |      |       |            | | Author          | Count | Percentage | | |
|         |       |            | |      |       |            | +=================+=======+============+ | |
|         |       |            | |      |       |            | | Matthias Kainer | 1     |            | | |
|         |       |            | |      |       |            | +-----------------+-------+------------+ | |
|         |       |            | +------+-------+------------+------------------------------------------+ |
+---------+-------+------------+--------------------------------------------------------------------------+
| authors | 1     | 25.00%     | +------+-------+------------+------------------------------------------+ |
|         |       |            | | Type | Count | Percentage | Author                                   | |
|         |       |            | +======+=======+============+==========================================+ |
|         |       |            | | feat | 1     | 100.00%    | +-----------------+-------+------------+ | |
|         |       |            | |      |       |            | | Author          | Count | Percentage | | |
|         |       |            | |      |       |            | +=================+=======+============+ | |
|         |       |            | |      |       |            | | Matthias Kainer | 1     |            | | |
|         |       |            | |      |       |            | +-----------------+-------+------------+ | |
|         |       |            | +------+-------+------------+------------------------------------------+ |
+---------+-------+------------+--------------------------------------------------------------------------+

```