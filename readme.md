### files-upload-gcp

#### Rust with google

how to use some google apis https://github.com/yoshidan/google-cloud-rust

google storage
https://github.com/yoshidan/google-cloud-rust/tree/main/storage

#### How to deploy

`just docker-build`

`just docker-push`

`just google-deploy`

### how to run

`cargo run`

go to http://localhost:8080/

```graphql
mutation GCPFile {
  is: upload_file(
    bucket: "sre_university_test"
    file_path: "my_files/test.txt"
    content: "some files"
  )
}
```

[google bucket](https://console.cloud.google.com/storage/browser/sre_university_test;tab=objects?forceOnBucketsSortingFiltering=true&project=cybx-chat&prefix=&forceOnObjectsSortingFiltering=false)
