require 'optparse'

def kitchen_instance(name, config)
  config = Kitchen::Config.new(config)
  config.colorize = false
  instances = config.instances
  return instances if name.nil? || name == 'all'
  instances.get(name)
end

def run_kitchen(action, name, loader_config = {})
  action = 'test' if action.nil?
  require 'kitchen'
  Kitchen.logger = Kitchen.default_file_logger
  config = { loader: Kitchen::Loader::YAML.new }
  kitchen_instance(name, config)
end

def data_path(instance)
  if instance.transport.send('config')[:username] == 'Administrator'
    'C:/Users/Administrator/AppData/Local/Temp/kitchen/data'
  else
    '/home/kitchen/prog/my-plans'
  end
end

def results(instance, pkg)
  if instance.transport.send('config')[:username] == 'Administrator'
    "habitat/#{pkg}/results"
  else
    "results"
  end
end

def last_build_env(instance, pkg)
  if instance.transport.send('config')[:username] == 'Administrator'
    "habitat/#{pkg}/results/last_build.ps1"
  else
    "results/last_build.env"
  end
end

def plans_path(instance)
  "#{data_path(instance)}/habitat/"
end

def sudo?(instance)
  if instance.transport.send('config')[:username] == 'centos'
    "sudo"
  elsif instance.transport.send('config')[:username] == 'root'
    "sudo"
  else
    ""
  end
end

