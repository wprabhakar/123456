#!/bin/bash
INPUT_FILE=../test.log
OUTPUT_FILE=../output.log
START_TIME="2019-06-10 00:00:00"
END_TIME="2019-06-19 23:59:59"
FMT="+%Y%m%d%H%M%S"
st=$(date -j -f "%Y-%m-%d %H:%M:%S" "${START_TIME}" "${FMT}")
et=$(date -j -f "%Y-%m-%d %H:%M:%S" "${END_TIME}" "${FMT}")
while read -r line;
do
  dt=`echo $line|awk '{print $4}' | sed 's/[^a-zA-Z0-9/:+]//g'`
  dtFormatted=$(date -j -f "%d/%b/%Y:%H:%M:%S" "$dt" "${FMT}")
  if [ "$dtFormatted" -lt "$st" ] ;
  then
   continue ;
  fi
  if [ "$dtFormatted" -gt "$et" ] ;
  then
   break ;
  fi
  echo "$dt $dtFormatted $st $et"
  echo "$line" >> ${OUTPUT_FILE}
done < ${INPUT_FILE}
echo "Created ${OUTPUT_FILE}"
echo "Top 10 Hosts"
cat ${OUTPUT_FILE} | awk '{print $1}' | sort | uniq -c | sort -r | head -n 10
