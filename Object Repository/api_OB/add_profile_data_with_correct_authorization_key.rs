<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description>API for adding profile data</description>
   <name>add_profile_data_with_correct_authorization_key</name>
   <tag></tag>
   <elementGuidId>d4f02129-ece3-4d9e-86a5-3565020b3332</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n\&quot;email\&quot;:\&quot;com\&quot;,\n\&quot;username\&quot;:\&quot;test\&quot;,\n\&quot;phone\&quot;:\&quot;1234556\&quot;,\n\&quot;image\&quot;: {\&quot;obj\&quot;:\&quot;res\&quot;}\n}\n&quot;,
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
      <value>VSM01234567891</value>
   </httpHeaderProperties>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>https://demo.getvsm.com/tone_analysis/api/test/add_profile/v1?=&amp;=&amp;=&amp;=</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
