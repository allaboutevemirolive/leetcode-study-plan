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

    for (const element of elements) {
        // Extract the text content from the element
        // const text = await element.textContent();

        // Click on the element
        await element.click();

        // Wait for the new page to load
        await page.waitForLoadState('networkidle');

        // Get the URL of the current page
        // const currentUrl = page.url();

        // Save the URL into a text file
        // const outputPath = `./${text}.txt`;
        // fs.writeFileSync(outputPath, currentUrl);

        // console.log(`Clicked on ${text}, URL saved to ${outputPath}`);

        // // Go back to the previous page
        // await page.goBack();

        // // Wait for the page to load
        // await page.waitForLoadState('networkidle');
        await new Promise(() => { });
    }

    // Close the browser
    // await browser.close();
    await new Promise(() => { });
})();
