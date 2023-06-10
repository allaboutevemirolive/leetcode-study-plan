const { chromium } = require('playwright');
const fs = require('fs');

(async () => {
    const browser = await chromium.launch( { headless: false });

    const context = await browser.newContext();

    const page = await context.newPage();

    await page.goto('https://leetcode.com/studyplan/top-interview-150/');

    await page.waitForLoadState('networkidle');

    const elements = await page.$$(
        'div[style="font-weight: 500; font-size: 14px;"]'
    );

    // Extract the text content from the elements
    const texts = await Promise.all(
        elements.map(async (element) => await element.textContent())
    );

    for (const text of texts) {
        await page.goto('https://leetcode.com/studyplan/top-interview-150/');

        // Find element with the specified text content
        // Click on the element

        await page.waitForTimeout(10 * 1000);

    }

    await browser.close();
})();
