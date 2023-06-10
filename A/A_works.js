const { chromium } = require('playwright');
const fs = require('fs');

(async () => {
    // Launch the browser
    const browser = await chromium.launch( { headless: false });

    // Create a new browser context
    const context = await browser.newContext();

    // Create a new page
    const page = await context.newPage();

    // Navigate to the target page
    await page.goto('https://leetcode.com/studyplan/top-interview-150/');

    // Wait for the page to load
    await page.waitForLoadState('networkidle');

    // Extract the elements with the specified style attribute
    const elements = await page.$$(
        'div[style="font-weight: 500; font-size: 14px;"]'
    );

    // Extract the text content from the elements
    const texts = await Promise.all(
        elements.map(async (element) => await element.textContent())
    );

    // Save the extracted data into a text file
    const outputPath = './extracted_data.txt';
    fs.writeFileSync(outputPath, texts.join('\n'));

    console.log(`Data extracted and saved to ${outputPath}`);

    // Close the browser
    await browser.close();
})();
