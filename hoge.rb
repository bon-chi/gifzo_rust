require 'rubygems'
require 'httpclient'

boundary = "123456"
c = HTTPClient.new
open("lorem_ipsum.txt") do |file|
    postdata = { "first_name" => "michael","file" => file }
    puts c.post_content("http://localhost:3000", postdata,
                         "content-type" => "multipart/form-data")
end
