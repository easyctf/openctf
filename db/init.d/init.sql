CREATE DATABASE `app`;
CREATE USER `app`@'%' IDENTIFIED BY 'hellosu';
GRANT ALL PRIVILEGES ON app.* TO app@'%';

CREATE DATABASE `minio`;
CREATE USER minio@'%' IDENTIFIED BY 'hellosu';
GRANT ALL PRIVILEGES ON minio.* TO minio@'%';
