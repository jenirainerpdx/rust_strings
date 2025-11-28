import { Octokit } from "@octokit/rest";
import OpenAI from "openai";

const prNumber = Number(process.env.PR_NUMBER);
const repoFull = process.env.REPO;
const [owner, repo] = repoFull.split("/");
const githubToken = process.env.GITHUB_TOKEN;
const openaiKey = process.env.OPENAI_API_KEY;

const octokit = new Octokit({ auth: githubToken });
const client = new OpenAI({ apiKey: openaiKey });

async function main() {
    // 1. Get PR info + diff
    const { data: pr } = await octokit.pulls.get({ owner, repo, pull_number: prNumber });
    const { data: diff } = await octokit.pulls.get({
        owner,
        repo,
        pull_number: prNumber,
        mediaType: { format: "diff" }
    });

    // Optionally truncate big diffs
    const MAX_CHARS = 12000;
    const truncatedDiff = diff.length > MAX_CHARS ? diff.slice(0, MAX_CHARS) : diff;

    // 2. Call OpenAI for review
    const systemPrompt = `
You are a senior ${pr.base.ref} code reviewer.
Focus on:
- correctness, bugs, race conditions
- security issues
- API design, readability, and maintainability
Be concise. Use bullet points. Reference line numbers when possible.
If things look good, still mention at least a couple of small suggestions or confirmations.
`;

    const userPrompt = `
Repository: ${owner}/${repo}
PR #${prNumber}: ${pr.title}

Description:
${pr.body || "(no description)"}

Diff (unified):
${truncatedDiff}
`;

    const response = await client.chat.completions.create({
        model: "gpt-4.1-mini",
        temperature: 0,
        messages: [
            { role: "system", content: systemPrompt },
            { role: "user", content: userPrompt }
        ]
    });

    const reviewBody = response.choices[0].message.content ?? "No feedback.";

    // 3. Post review as a PR comment
    await octokit.issues.createComment({
        owner,
        repo,
        issue_number: prNumber,
        body: `🤖 AI Code Review (ChatGPT)\n\n${reviewBody}`
    });
}

main().catch(err => {
    console.error(err);
    process.exit(1);
});
