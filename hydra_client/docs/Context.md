# Context

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**access_token_extra** | [**::std::collections::HashMap<String, Value>**](Value.md) | Extra represents arbitrary session data. | [optional] [default to null]
**client_id** | **String** | ClientID is id of the client the token was issued for.. | [optional] [default to null]
**expires_at** | **String** | ExpiresAt is the expiry timestamp. | [optional] [default to null]
**granted_scopes** | **Vec<String>** | GrantedScopes is a list of scopes that the subject authorized when asked for consent. | [optional] [default to null]
**issued_at** | **String** | IssuedAt is the token creation time stamp. | [optional] [default to null]
**issuer** | **String** | Issuer is the id of the issuer, typically an hydra instance. | [optional] [default to null]
**subject** | **String** | Subject is the identity that authorized issuing the token, for example a user or an OAuth2 app. This is usually a uuid but you can choose a urn or some other id too. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


