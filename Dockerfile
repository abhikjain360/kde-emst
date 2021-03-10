FROM nvidia/cuda:11.2.1-devel-ubuntu20.04

RUN ln -snf /usr/share/zoneinfo/Asia/Kolkata /etc/localtime && echo "Asia/Kolkata" > /etc/timezone

RUN apt update -y && apt upgrade -y

RUN apt install gnupg2 ca-certificates apt-utils software-properties-common -y ; apt-key adv --fetch-key https://repo.arrayfire.com/GPG-PUB-KEY-ARRAYFIRE-2020.PUB ; echo "deb [arch=amd64] https://repo.arrayfire.com/ubuntu focal main" | tee /etc/apt/sources.list.d/arrayfire.list ; apt update -y && apt upgrade -y && apt install arrayfire arrayfire-dev -y
