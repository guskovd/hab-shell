from alpine

ARG HAB_VERSION=0.61.0-20180815171844
ARG HAB_BINARY_PATH=/tmp/hab-$HAB_VERSION-x86_64-linux/hab
ARG HAB_TGZ=hab-$HAB_VERSION-x86_64-linux.tar.gz

RUN apk --update add openssh curl sudo bash rsync docker

RUN ssh-keygen -f /etc/ssh/ssh_host_rsa_key -N '' -t rsa
RUN ssh-keygen -f /etc/ssh/ssh_host_dsa_key -N '' -t dsa

RUN mkdir -p /var/run/sshd
RUN adduser -h /home/<%= @username %> -D -s /bin/bash <%= @username %>
RUN echo <%= "#{@username}:#kitchen" %> | chpasswd
RUN echo '<%= @username %> ALL=(ALL) NOPASSWD:ALL' >> /etc/sudoers
RUN mkdir -p /home/<%= @username %>/.ssh
RUN chown -R <%= @username %> /home/<%= @username %>/.ssh
RUN chmod 0700 /home/<%= @username %>/.ssh
RUN touch /home/<%= @username %>/.ssh/authorized_keys
RUN chown <%= @username %> /home/<%= @username %>/.ssh/authorized_keys
RUN chmod 0600 /home/<%= @username %>/.ssh/authorized_keys
RUN echo '<%= IO.read(@public_key).strip %>' >> /home/<%= @username %>/.ssh/authorized_keys

RUN sed -i 's/root:\/bin\/ash/root:\/bin\/bash/' /etc/passwd

RUN wget https://dl.bintray.com/habitat/stable/linux/x86_64/$HAB_TGZ -O /tmp/$HAB_TGZ
RUN mkdir -p /hab/bin
RUN mkdir -p /tmp
RUN cd /tmp && \
    tar -xzf $HAB_TGZ && \
    cp $HAB_BINARY_PATH /bin

ENV PATH /hab/local/bin:/hab/bin:$PATH

CMD /usr/sbin/sshd -D -o UseDNS=no -o UsePAM=no -o PasswordAuthentication=yes -o UsePrivilegeSeparation=no -o PidFile=/tmp/sshd.pid
