<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description>API for updating the profile using correct authorization key</description>
   <name>update_profile_data_with_correct_authorization_key</name>
   <tag></tag>
   <elementGuidId>ab0f1bdd-17b5-4631-a1ba-e90758bdd0ef</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n\&quot;email\&quot;:\&quot;test@getvsm.com\&quot;,\n\&quot;username\&quot;:\&quot;test\&quot;,\n\&quot;phone\&quot;:\&quot;1234\&quot;,\n\&quot;image\&quot;: {\&quot;obj\&quot;:\&quot;res\&quot;}\n}\n\n&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>VSM01234567890</value>
   </httpHeaderProperties>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>PUT</restRequestMethod>
   <restUrl>https://demo.getvsm.com/tone_analysis/api/test/update_profile/v1</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
