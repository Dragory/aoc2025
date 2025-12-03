if (!String(process.env.npm_config_user_agent).startsWith("pnpm/")) {
    console.error("ERROR: Only pnpm is supported in this project");
    process.exit(1);
}
