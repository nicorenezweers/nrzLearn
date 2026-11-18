using Base64, Logging
const SNIPS = [
SNIPS_HERE
]
const REAL_STDOUT = stdout
for (i, s) in enumerate(SNIPS)
    code = String(base64decode(s))
    path, io = mktemp()
    errbuf = IOBuffer()
    ok = true
    redirect_stdout(io) do
        with_logger(SimpleLogger(errbuf)) do
            try
                include_string(Module(:Main), code, "main.jl")
            catch e
                ok = false
                showerror(errbuf, e)
            end
        end
    end
    flush(io)
    close(io)
    out = read(path, String)
    rm(path)
    println(REAL_STDOUT, "@@ ", i, " ", ok, " ", base64encode(out), " ", base64encode(String(take!(errbuf))))
end
