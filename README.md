# OpenDAL WebDAV Server

A WebDAV server powered by [Apache OpenDAL](https://opendal.apache.org/), enabling WebDAV access to any storage backend supported by OpenDAL.

## Supported Backends

- Amazon S3 and compatible services
- Azure Blob Storage
- Azure Data Lake Storage
- Google Cloud Storage
- Alibaba Cloud OSS
- Huawei Cloud OBS
- Tencent Cloud COS
- Local filesystem
- Memory (for testing)
- And more...

## Usage

Run with Docker

```bash
docker run -p 8080:8080 \
  -e OPENDAL_TYPE=s3 \
  -e OPENDAL_BUCKET=mybucket \
  -e OPENDAL_REGION=us-east-1 \
  -e OPENDAL_ACCESS_KEY_ID=xxx \
  -e OPENDAL_SECRET_ACCESS_KEY=xxx \
  ghcr.io/frostming/opendal-webdav-server:latest
```

### Environment Variables

| Variable       | Description                                              | Required           |
| -------------- | -------------------------------------------------------- | ------------------ |
| `OPENDAL_TYPE` | Storage backend type (e.g., `s3`, `fs`, `gcs`, `azblob`) | Yes                |
| `OPENDAL_*`    | Backend-specific configuration (see below)               | Depends on backend |
| `HOST`         | Listen address (default: `0.0.0.0`)                      | No                 |
| `PORT`         | Listen port (default: `8080`)                            | No                 |

Configuration variables are automatically converted: `OPENDAL_ACCESS_KEY_ID` becomes `access-key-id` in OpenDAL config.

### Examples

**Local Filesystem:**

```bash
OPENDAL_TYPE=fs \
OPENDAL_ROOT=/data \
cargo run
```

**Amazon S3:**

```bash
OPENDAL_TYPE=s3 \
OPENDAL_BUCKET=mybucket \
OPENDAL_REGION=us-east-1 \
OPENDAL_ACCESS_KEY_ID=your-access-key \
OPENDAL_SECRET_ACCESS_KEY=your-secret-key \
cargo run
```

**S3-Compatible (MinIO, etc.):**

```bash
OPENDAL_TYPE=s3 \
OPENDAL_BUCKET=mybucket \
OPENDAL_ENDPOINT=http://localhost:9000 \
OPENDAL_ACCESS_KEY_ID=minioadmin \
OPENDAL_SECRET_ACCESS_KEY=minioadmin \
cargo run
```

**Google Cloud Storage:**

```bash
OPENDAL_TYPE=gcs \
OPENDAL_BUCKET=mybucket \
OPENDAL_CREDENTIAL=/path/to/service-account.json \
cargo run
```

**Azure Blob Storage:**

```bash
OPENDAL_TYPE=azblob \
OPENDAL_CONTAINER=mycontainer \
OPENDAL_ACCOUNT_NAME=myaccount \
OPENDAL_ACCOUNT_KEY=mykey \
cargo run
```

## Connecting

Once running, connect using any WebDAV client:

- **macOS Finder:** Go > Connect to Server > `http://localhost:8080`
- **Windows Explorer:** Map Network Drive > `http://localhost:8080`
- **Linux:** `davfs2`, `cadaver`, or file manager with WebDAV support
- **rclone:** `rclone config` with WebDAV type

## License

MIT License - see [LICENSE](LICENSE) for details.
