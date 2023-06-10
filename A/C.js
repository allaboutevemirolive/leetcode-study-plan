const { chromium } = require('playwright');
const fs = require('fs');

(async () => {
    const browser = await chromium.launch({ headless: false });

    // Create a new browser context
    const context = await browser.newContext();

    const page = await context.newPage();

    await page.goto('https://leetcode.com/studyplan/top-interview-150/');

    // await page.waitForLoadState('networkidle');
    await page.waitForTimeout( 10 * 1000 );

    const elements = await page.$$(
        'div[style="font-weight: 500; font-size: 14px;"]'
    );

    for (const element of elements) {
        await element.click();

        // await page.waitForLoadState('networkidle');

        await page.waitForTimeout( 10 * 1000 );

        await page.goto('https://leetcode.com/studyplan/top-interview-150/');
    }

    await new Promise(() => { });
})();
