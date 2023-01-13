FROM rust:1.66

RUN apt update && apt install curl openssh-server sudo git -y
RUN useradd -rm -d /home/ubuntu -s /bin/bash -g root -G sudo -u 1000 test
RUN  echo 'test:test' | chpasswd
RUN service ssh start
WORKDIR /home/ubuntu

EXPOSE 22
CMD ["/usr/sbin/sshd","-D"]