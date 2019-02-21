require_relative 'lib/kitchen_helper.rb'
require 'optparse'

task :diagnose, [:name] do |_t, args|
  p run_kitchen(:name, args.name).diagnose
end

task :build, [:name, :pkg] do |_t, args|
  i = run_kitchen(:name, args.name)
  hostname = i.send('state_file').read[:hostname]
  if i.transport.send('config')[:username] == 'root'
    system <<-EOH
      docker run --rm --privileged -ti -e HAB_NOCOLORING=true -e HAB_ORIGIN=guskovd -v hab_pkgs:/hab/pkgs -v hab_studios:/hab/studios -v $PWD:/src -v $HOME/.hab/cache/keys:/hab/cache/keys -v $HOME/.hab/cache/artifacts:/hab/cache/artifacts -v /var/run/docker.sock:/var/run/docker.sock -w /src dguskov/doha:base hab studio build -R habitat/#{args.pkg}
    EOH
  elsif i.transport.send('config')[:username] == 'Administrator'
    system "rsync -qrazc ./habitat rsync://#{hostname}/#{data_path i}/"
    i.remote_exec <<-EOH
      export TEMP='C:/cygwin/tmp'
      cd #{data_path i}
      hab studio build -R habitat/#{args.pkg}
    EOH
  end
end

task :enter, [:name, :pkg] do |_t, args|
  i = run_kitchen(:name, args.name)
  i.remote_exec <<-EOH
    export TEMP='C:/cygwin/tmp'
    cd #{data_path i}
    hab studio enter habitat/#{args.pkg}
  EOH
end

task :travis, [:name, :pkg] do |_t, args|
  i = run_kitchen(:name, args.name)
  hostname = i.send('state_file').read[:hostname]
  system "rsync -qrazc $PWD/ rsync://#{hostname}/#{data_path i}"
  i.remote_exec <<-EOH
    cd #{data_path i}
    export HAB_PKG=#{args.pkg}
    ./travis/build.sh
  EOH
end

task :install, [:name, :pkg] do |_t, args|
  i = run_kitchen(:name, args.name)
  i.remote_exec <<-EOH
    pushd #{data_path i} > /dev/null
    pkg_artifact=$(cat #{last_build_env(i, args.pkg)} | grep pkg_artifact | awk -F '=' '{print $2}' | sed $'s/[\r:\"]//g')
    #{sudo? i} hab pkg install "#{results(i, args.pkg)}/$pkg_artifact"
  EOH
end

task :upload, [:name, :pkg] do |_t, args|
  i = run_kitchen(:name, args.name)
  i.remote_exec <<-EOH
    pushd #{data_path i} > /dev/null
    pkg_artifact=$(cat #{last_build_env(i, args.pkg)} | grep pkg_artifact | awk -F '=' '{print $2}' | sed $'s/[\r:\"]//g')
    hab pkg upload "#{results(i, args.pkg)}/$pkg_artifact"
  EOH
end

task :promote, [:name, :pkg] do |_t, args|
  i = run_kitchen(:name, args.name)
  i.remote_exec <<-EOH
     pushd #{data_path i} > /dev/null
     pkg_ident=$(cat #{last_build_env(i, args.pkg)} | grep pkg_ident | awk -F '=' '{print $2}' | sed $'s/[\r:\"]//g') 
     hab pkg promote $pkg_ident stable
  EOH
end
