INPUT_FILE=../test.log
MAX_TRAFFIC_IP=$(cat ${INPUT_FILE} | grep "GET \|POST \|PUT \|OPTION \|CONNECT \|HEAD \|PATCH \|DELETE \|TRACE \|PROPFIND " | awk '{print $1}' | sort | uniq -c | sort -r | head -n 1 | awk '{print $2}')
COUNTRY_CODE=$(curl -s -X POST -d "return=array&ip=${MAX_TRAFFIC_IP}" http://ip2country.hackers.lv/api/ip2country | tr -d '"[]')
echo "Country Code: ${COUNTRY_CODE}"
curl -s https://restcountries.com/v3.1/alpha/${COUNTRY_CODE} | jq '.[].name.common'
