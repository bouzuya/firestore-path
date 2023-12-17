# firestore-path

A Firestore path helper.

## NOTE: Naming

<https://firebase.google.com/docs/firestore/data-model>

> For convenience, you can also create references by specifying the path to a document or collection as a string, with path components separated by a forward slash (/). For example, to create a reference to the alovelace document:

- `the path to a document or collection`
- `path components`

<https://firebase.google.com/docs/firestore/reference/rest/v1/projects.databases.documents/get>

- `name`: The resource name of the document.
  - format: `projects/{projectId}/databases/{databaseId}/documents/{document_path}`
  - `document_path` は `documents/...` に続く箇所を表している

<https://firebase.google.com/docs/firestore/reference/rest/v1/projects.databases.documents/list>

- `parent`: The parent resource name
  - format: `projects/{projectId}/databases/{databaseId}/documents`
    - example: `projects/my-project/databases/my-database/documents`
  - format: `projects/{projectId}/databases/{databaseId}/documents/{document_path}`
    - example: `projects/my-project/databases/my-database/documents/chatrooms/my-chatroom`
- `collectionId`: The collection ID
  - example: `chatrooms`, `messages`

<https://firebase.google.com/docs/firestore/reference/rest/v1/projects.databases.documents/partitionQuery>

- database resource name
  - format: `projects/{projectId}/databases/{databaseId}/documents`

<https://firebase.google.com/docs/firestore/reference/rest/v1/projects.databases/create>

> This value should be 4-63 characters. Valid characters are `/[a-z][0-9]-/` with first character a letter and the last a letter or a number. Must not be UUID-like `/[0-9a-f]{8}(-[0-9a-f]{4}){3}-[0-9a-f]{12}/`.

- `databaseId`
  - format: `This value should be 4-63 characters. Valid characters are /[a-z][0-9]-/ with first character a letter and the last a letter or a number. Must not be UUID-like /[0-9a-f]{8}(-[0-9a-f]{4}){3}-[0-9a-f]{12}/.`

<https://firebase.google.com/docs/reference/js/firestore_.md#collection>

- `path`
- `CollectionReference`

<https://firebase.google.com/docs/reference/js/firestore_.documentreference>

- `DocumentReference`

### Reference

- Cloud Firestore Data model
  <https://firebase.google.com/docs/firestore/data-model>
- Cloud Firestore API (REST)
  <https://firebase.google.com/docs/firestore/reference/rest>
- JavaScript API reference
  <https://firebase.google.com/docs/reference/js/firestore_>
