#!/usr/bin/env ruby
require "time"
require "logger"

LEVELS = %w[TRACE DEBUG INFO WARN ERROR CRITICAL FATAL].freeze
ACTIONS = [
  "processing request",
  "connecting to database",
  "fetching user data",
  "updating cache",
  "validating input",
  "generating report",
  "sending notification"
].freeze
USERS = %w[user123 admin guest service_account api_client].freeze
ERRORS = [
  "connection timeout",
  "invalid credentials",
  "resource not found",
  "permission denied",
  "internal server error"
].freeze

def parse_interval(value)
  return 1.0 if value.nil? || value.strip.empty?

  v = value.strip.downcase
  number = v[/\A\d+(\.\d+)?/]
  return 1.0 if number.nil?

  amount = number.to_f
  suffix = v[number.length..]&.strip

  case suffix
  when "ms" then amount / 1000.0
  when "s" then amount
  when "m" then amount * 60.0
  when "h" then amount * 3600.0
  else 1.0
  end
end

def parse_total(value)
  return -1 if value.nil? || value.strip.empty?

  Integer(value)
rescue ArgumentError
  -1
end

def build_message(level)
  action = ACTIONS.sample
  user = USERS.sample
  error = ERRORS.sample

  case level
  when "TRACE" then "trace #{action}"
  when "DEBUG" then "debug #{action}"
  when "INFO" then "info #{action} user=#{user}"
  when "WARN" then "warn possible issue action=#{action}"
  else "error #{error} user=#{user}"
  end
end

interval = parse_interval(ENV["LOG_INTERVAL"])
total = parse_total(ENV["TOTAL_LOGS"])
logger = Logger.new($stdout)
logger.level = Logger::DEBUG
logger.formatter = proc do |severity, datetime, _progname, msg|
  "#{datetime.utc.iso8601} #{severity} language=ruby #{msg}\n"
end

seq = 1
while total == -1 || seq <= total
  level = LEVELS.sample
  message = build_message(level)

  line = "seq=#{seq} request_id=- duration_ms=- msg=\"#{message}\""
  line += " request_id=#{rand(10_000)}" if rand < 0.5
  line += format(" duration_ms=%.2f", rand * 1000.0) if rand < 0.3

  case level
  when "TRACE", "DEBUG" then logger.debug(line)
  when "INFO" then logger.info(line)
  when "WARN" then logger.warn(line)
  else logger.error(line)
  end
  seq += 1
  sleep(interval)
end

