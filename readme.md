# rgitstats

An application that generates simple stats from your conventional commits

## Install

```bash
RELEASE_VERSION=0.1.1
RELEASE_OS=x86_64-unknown-linux-gnu # or x86_64-apple-darwin
curl -L -o rgitstats https://github.com/MatthiasKainer/rgitstats/releases/download/$RELEASE_VERSION/rgitstats-$RELEASE_VERSION-$RELEASE_OS
chmod +x rgitstats
./rgitstats path/to/my/repo
```

You can also move it to you bin path from there to have it globally available

## Usage

```bash
rgiststats path/to/my/repo

+------------+-------+------------+
| Type       | Count | Percentage |
+============+=======+============+
| feat       | 168   | 43.98%     |
+------------+-------+------------+
| chore      | 102   | 26.70%     |
+------------+-------+------------+
| fix        | 69    | 18.06%     |
+------------+-------+------------+
| merge      | 21    | 5.50%      |
+------------+-------+------------+
| ci         | 9     | 2.36%      |
+------------+-------+------------+
| bug        | 6     | 1.57%      |
+------------+-------+------------+
| chores     | 3     | 0.79%      |
+------------+-------+------------+
| tests      | 1     | 0.26%      |
+------------+-------+------------+
| feat/      | 1     | 0.26%      |
+------------+-------+------------+
| revert     | 1     | 0.26%      |
+------------+-------+------------+
```