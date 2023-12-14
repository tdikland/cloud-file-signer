# Cloud File Signer

This crate defines a generic interface for signing cloud object store URLs.

A presigned URL is a URL that grants temporary access to a file in a cloud object store. The URL is signed by the cloud provider and can be used to access the file without further authentication. 


![Maintenance](https://img.shields.io/badge/maintenance-actively--developed-brightgreen.svg)
[![main](https://github.com/tdikland/cloud-file-signer/actions/workflows/build.yml/badge.svg?branch=main)](https://github.com/tdikland/cloud-file-signer/actions/workflows/main.yml)
[![crates-io](https://img.shields.io/crates/v/cloud-file-signer.svg)](https://crates.io/crates/cloud-file-signer)
[![api-docs](https://docs.rs/cloud-file-signer/badge.svg)](https://docs.rs/cloud-file-signer)

## Supported Object Stores

This crate provides an implementation of the `CloudFileSigner` trait for the following object stores.

| Object Store         | Status             |
|:--------------------:|:------------------:|
| Amazon S3            | :white_check_mark: |
| Azure Blob Storage   | :white_check_mark: |
| Google Cloud Storage | :white_check_mark: |
