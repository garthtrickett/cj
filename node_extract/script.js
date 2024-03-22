const fs = require("fs");
const https = require("https");
const jsdom = require("jsdom");
const { JSDOM } = jsdom;

let data = [];

// Read the existing data from output.json
if (fs.existsSync("output.json")) {
  const jsonData = fs.readFileSync("output.json", "utf-8");
  data = JSON.parse(jsonData);
}

function delay(ms) {
  return new Promise((resolve) => setTimeout(resolve, ms));
}

async function scrape() {
  for (let i = 1; i <= 999; i++) {
    // Skip if the id already exists in the data
    if (data.some((item) => item.id === i)) {
      console.log(`Skipping URL ${i} of 999: Already processed`);
      continue;
    }

    let url = `https://bunpro.jp/grammar_points/${i}`;

    // Print out the current number
    console.log(`Processing URL ${i} of 999: ${url}`);

    https
      .get(url, (res) => {
        let body = [];
        res.on("data", (chunk) => body.push(chunk));
        res.on("end", () => {
          try {
            body = Buffer.concat(body).toString();
            const dom = new JSDOM(body);

            // Get the div with id="structure"
            let div = dom.window.document.querySelector("div#structure");

            // Get the next sibling which is a <p>
            let p = div.nextElementSibling;
            while (p && p.tagName.toLowerCase() !== "p") {
              p = p.nextElementSibling;
            }

            // Get the stripped text
            let strippedText = p
              ? p.innerHTML.replace(/<(?!br\s*\/?)[^>]+>/g, "")
              : "";

            // Get the div with id="about"
            let div_about = dom.window.document.querySelector("div#about");

            // Get the child h3
            let h3 = div_about.querySelector("h3");

            // Get the text and remove "About"
            let text = h3 ? h3.textContent.replace("About ", "") : "";

            // Get the h4 with text "Grammar Info"
            let h4 = Array.from(
              dom.window.document.querySelectorAll("h4"),
            ).find((el) => el.textContent.trim() === "Grammar Info");

            // Get the next sibling which is a <p>
            let p_lesson = h4 ? h4.nextElementSibling : null;
            while (p_lesson && p_lesson.tagName.toLowerCase() !== "p") {
              p_lesson = p_lesson.nextElementSibling;
            }

            // Get the text inside of it
            let lesson = p_lesson ? p_lesson.textContent : "";

            // Add the lesson to the data array
            data.push({
              id: i,
              rule: strippedText,
              structure: text,
              lesson: lesson,
            });

            // Write the data to a JSON file
            fs.writeFileSync("output.json", JSON.stringify(data, null, 2));
          } catch (error) {
            console.error(
              `An error occurred while processing URL ${url}: ${error.message}`,
            );
          }
        });
      })
      .on("error", (error) => {
        console.error(
          `An error occurred while downloading URL ${url}: ${error.message}`,
        );
      });

    // Wait for 2 seconds
    await delay(2000);
  }
}

scrape();
