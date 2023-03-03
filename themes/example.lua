-- use("git")
-- use("aws")
-- use("env")
--

-- rust calls this to know what to load
function Modules()
	return {
		"git",
		"aws",
		"env",
		"weather",
	}
end

function Prompt(ctx)
	local result = ""
	if ctx.git.ok then
		local git = ctx.git
        for k,v in pairs(git) do
            print("git", k, ": ", v)
        end
	end
	result = result .. ctx.env.AWS_PROFILE
	result = result .. ctx.username
	return result
end
