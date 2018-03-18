# ConsentRequest

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**client_id** | **String** | ClientID is the client id that initiated the OAuth2 request. | [optional] [default to null]
**id** | **String** | ID is the id of this consent request. | [optional] [default to null]
**redirect_url** | **String** | Redirect URL is the URL where the user agent should be redirected to after the consent has been accepted or rejected. | [optional] [default to null]
**requested_acr** | **Vec<String>** |  | [optional] [default to null]
**requested_max_age** | **i64** |  | [optional] [default to null]
**requested_prompt** | **String** |  | [optional] [default to null]
**requested_scopes** | **Vec<String>** | RequestedScopes represents a list of scopes that have been requested by the OAuth2 request initiator. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


