require 'ffi'

module AddArray
    extend FFI::Library
    ffi_lib File.expand_path("target/release/libaddarray.dylib")
    attach_function :add_array, [:uint64, :uint64], :uint64
end

puts AddArray::add_array(ARGV[0].to_i, ARGV[1].to_i)