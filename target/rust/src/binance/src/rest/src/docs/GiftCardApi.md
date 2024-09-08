# \GiftCardApi

All URIs are relative to *https://api.binance.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**sapi_v1_giftcard_buy_code_post**](GiftCardApi.md#sapi_v1_giftcard_buy_code_post) | **POST** /sapi/v1/giftcard/buyCode | Buy a Binance Code (TRADE)
[**sapi_v1_giftcard_buy_code_token_limit_get**](GiftCardApi.md#sapi_v1_giftcard_buy_code_token_limit_get) | **GET** /sapi/v1/giftcard/buyCode/token-limit | Fetch Token Limit (USER_DATA)
[**sapi_v1_giftcard_create_code_post**](GiftCardApi.md#sapi_v1_giftcard_create_code_post) | **POST** /sapi/v1/giftcard/createCode | Create a Binance Code (USER_DATA)
[**sapi_v1_giftcard_cryptography_rsa_public_key_get**](GiftCardApi.md#sapi_v1_giftcard_cryptography_rsa_public_key_get) | **GET** /sapi/v1/giftcard/cryptography/rsa-public-key | Fetch RSA Public Key (USER_DATA)
[**sapi_v1_giftcard_redeem_code_post**](GiftCardApi.md#sapi_v1_giftcard_redeem_code_post) | **POST** /sapi/v1/giftcard/redeemCode | Redeem a Binance Code (USER_DATA)
[**sapi_v1_giftcard_verify_get**](GiftCardApi.md#sapi_v1_giftcard_verify_get) | **GET** /sapi/v1/giftcard/verify | Verify a Binance Code (USER_DATA)



## sapi_v1_giftcard_buy_code_post

> models::SapiV1GiftcardCreateCodePost200Response sapi_v1_giftcard_buy_code_post(base_token, face_token, base_token_amount, timestamp, signature, recv_window)
Buy a Binance Code (TRADE)

This API is for buying a fixed-value Binance Code, which means your Binance Code will be redeemable to a token that is different to the token that you are paying in. If the token you’re paying and the redeemable token are the same, please use the Create Binance Code endpoint. You can use supported crypto currency or fiat token as baseToken to buy Binance Code that is redeemable to your chosen faceToken. Once successfully purchased, the amount of baseToken would be deducted from your funding wallet.  To get started with, please make sure: - You have a Binance account - You have passed kyc - You have a sufficient balance in your Binance funding wallet - You need Enable Withdrawals for the API Key which requests this endpoint.  Daily creation volume: 2 BTC / 24H Daily creation times: 200 Codes / 24H  Weight(IP): 1

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**base_token** | **String** | The token you want to pay, example BUSD | [required] |
**face_token** | **String** | The token you want to buy, example BNB. If faceToken = baseToken, it's the same as createCode endpoint. | [required] |
**base_token_amount** | **f64** | The base token asset quantity, example  1.002 | [required] |
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**models::SapiV1GiftcardCreateCodePost200Response**](_sapi_v1_giftcard_createCode_post_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_giftcard_buy_code_token_limit_get

> models::SapiV1GiftcardBuyCodeTokenLimitGet200Response sapi_v1_giftcard_buy_code_token_limit_get(base_token, timestamp, signature, recv_window)
Fetch Token Limit (USER_DATA)

This API is to help you verify which tokens are available for you to purchase fixed-value gift cards as mentioned in section 2 and it's limitation.  Weight(IP): 1

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**base_token** | **String** | The token you want to pay, example BUSD | [required] |
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**models::SapiV1GiftcardBuyCodeTokenLimitGet200Response**](_sapi_v1_giftcard_buyCode_token_limit_get_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_giftcard_create_code_post

> models::SapiV1GiftcardCreateCodePost200Response sapi_v1_giftcard_create_code_post(token, amount, timestamp, signature, recv_window)
Create a Binance Code (USER_DATA)

This API is for creating a Binance Code. To get started with, please make sure:  - You have a Binance account - You have passed kyc - You have a sufficient balance in your Binance funding wallet - You need Enable Withdrawals for the API Key which requests this endpoint.  Daily creation volume: 2 BTC / 24H Daily creation times: 200 Codes / 24H  Weight(IP): 1

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | The coin type contained in the Binance Code | [required] |
**amount** | **f64** | The amount of the coin | [required] |
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**models::SapiV1GiftcardCreateCodePost200Response**](_sapi_v1_giftcard_createCode_post_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_giftcard_cryptography_rsa_public_key_get

> models::SapiV1GiftcardCryptographyRsaPublicKeyGet200Response sapi_v1_giftcard_cryptography_rsa_public_key_get(timestamp, signature, recv_window)
Fetch RSA Public Key (USER_DATA)

This API is for fetching the RSA Public Key. This RSA Public key will be used to encrypt the card code. Please note that the RSA Public key fetched is valid only for the current day.  Weight(IP): 1

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**models::SapiV1GiftcardCryptographyRsaPublicKeyGet200Response**](_sapi_v1_giftcard_cryptography_rsa_public_key_get_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_giftcard_redeem_code_post

> models::SapiV1GiftcardRedeemCodePost200Response sapi_v1_giftcard_redeem_code_post(code, timestamp, signature, external_uid, recv_window)
Redeem a Binance Code (USER_DATA)

This API is for redeeming the Binance Code. Once redeemed, the coins will be deposited in your funding wallet.  Please note that if you enter the wrong code 5 times within 24 hours, you will no longer be able to redeem any Binance Code that day.  Weight(IP): 1

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**code** | **String** | Binance Code | [required] |
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**external_uid** | Option<**String**> | Each external unique ID represents a unique user on the partner platform. The function helps you to identify the redemption behavior of different users, such as redemption frequency and amount. It also helps risk and limit control of a single account, such as daily limit on redemption volume, frequency, and incorrect number of entries. This will also prevent a single user account reach the partner's daily redemption limits. We strongly recommend you to use this feature and transfer us the User ID of your users if you have different users redeeming Binance codes on your platform. To protect user data privacy, you may choose to transfer the user id in any desired format (max. 400 characters). |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**models::SapiV1GiftcardRedeemCodePost200Response**](_sapi_v1_giftcard_redeemCode_post_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_giftcard_verify_get

> models::SapiV1GiftcardVerifyGet200Response sapi_v1_giftcard_verify_get(reference_no, timestamp, signature, recv_window)
Verify a Binance Code (USER_DATA)

This API is for verifying whether the Binance Code is valid or not by entering Binance Code or reference number.  Please note that if you enter the wrong binance code 5 times within an hour, you will no longer be able to verify any binance code for that hour.  Weight(IP): 1

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**reference_no** | **String** | reference number | [required] |
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**models::SapiV1GiftcardVerifyGet200Response**](_sapi_v1_giftcard_verify_get_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

