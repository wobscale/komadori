# OAuth2Client

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**client_name** | **String** | Name is the human-readable string name of the client to be presented to the end-user during authorization. | [optional] 
**client_secret** | **String** | Secret is the client&#39;s secret. The secret will be included in the create request as cleartext, and then never again. The secret is stored using BCrypt so it is impossible to recover it. Tell your users that they need to write the secret down as it will not be made available again. | [optional] 
**client_uri** | **String** | ClientURI is an URL string of a web page providing information about the client. If present, the server SHOULD display this URL to the end-user in a clickable fashion. | [optional] 
**contacts** | **Vec<String>** | Contacts is a array of strings representing ways to contact people responsible for this client, typically email addresses. | [optional] 
**grant_types** | **Vec<String>** | GrantTypes is an array of grant types the client is allowed to use. | [optional] 
**id** | **String** | ID is the id for this client. | [optional] 
**logo_uri** | **String** | LogoURI is an URL string that references a logo for the client. | [optional] 
**owner** | **String** | Owner is a string identifying the owner of the OAuth 2.0 Client. | [optional] 
**policy_uri** | **String** | PolicyURI is a URL string that points to a human-readable privacy policy document that describes how the deployment organization collects, uses, retains, and discloses personal data. | [optional] 
**public** | **bool** | Public is a boolean that identifies this client as public, meaning that it does not have a secret. It will disable the client_credentials grant type for this client if set. | [optional] 
**redirect_uris** | **Vec<String>** | RedirectURIs is an array of allowed redirect urls for the client, for example http://mydomain/oauth/callback . | [optional] 
**response_types** | **Vec<String>** | ResponseTypes is an array of the OAuth 2.0 response type strings that the client can use at the authorization endpoint. | [optional] 
**scope** | **String** | Scope is a string containing a space-separated list of scope values (as described in Section 3.3 of OAuth 2.0 [RFC6749]) that the client can use when requesting access tokens. | [optional] 
**tos_uri** | **String** | TermsOfServiceURI is a URL string that points to a human-readable terms of service document for the client that describes a contractual relationship between the end-user and the client that the end-user accepts when authorizing the client. | [optional] 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


