# WardenTokenAccessRequestResponse

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**access_token_extra** | [**::std::collections::HashMap<String, Value>**](Value.md) | Extra represents arbitrary session data. | [optional] 
**allowed** | **bool** | Allowed is true if the request is allowed and false otherwise. | [optional] 
**client_id** | **String** | ClientID is the id of the OAuth2 client that requested the token. | [optional] 
**expires_at** | **String** | ExpiresAt is the expiry timestamp. | [optional] 
**granted_scopes** | **Vec<String>** | GrantedScopes is a list of scopes that the subject authorized when asked for consent. | [optional] 
**issued_at** | **String** | IssuedAt is the token creation time stamp. | [optional] 
**issuer** | **String** | Issuer is the id of the issuer, typically an hydra instance. | [optional] 
**subject** | **String** | Subject is the identity that authorized issuing the token, for example a user or an OAuth2 app. This is usually a uuid but you can choose a urn or some other id too. | [optional] 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


