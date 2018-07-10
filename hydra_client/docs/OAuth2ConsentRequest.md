# OAuth2ConsentRequest

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**client_id** | **String** | ClientID is the client id that initiated the OAuth2 request. | [optional] 
**expires_at** | **String** | ExpiresAt is the time where the access request will expire. | [optional] 
**id** | **String** | ID is the id of this consent request. | [optional] 
**redirect_url** | **String** | Redirect URL is the URL where the user agent should be redirected to after the consent has been accepted or rejected. | [optional] 
**requested_scopes** | **Vec<String>** | RequestedScopes represents a list of scopes that have been requested by the OAuth2 request initiator. | [optional] 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


