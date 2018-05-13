#!/usr/bin/env ruby
# encoding: utf-8

def font
  $font ||= IO.read("src/font.rs").scan(/0x[0-9A-F]+/).map{|c| Integer(c)}
end

def render_char(c)
  font[c]
    .to_s(2)
    .rjust(64, '0')
    .split(//)
    .each_slice(8)
    .map(&:to_a)
    .map(&:join)
    .join("\n")
    .gsub(/0/, ' ')
    .gsub(/1/, '*')
end

ARGV.first.each_char do |c|
  puts render_char(c.ord)
end
