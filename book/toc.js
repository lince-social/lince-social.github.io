// Populate the sidebar
//
// This is a script, and not included directly in the page, to control the total size of the book.
// The TOC contains an entry for each page, so if each page includes a copy of the TOC,
// the total size of the page becomes O(n**2).
class MDBookSidebarScrollbox extends HTMLElement {
    constructor() {
        super();
    }
    connectedCallback() {
        this.innerHTML = '<ol class="chapter"><li class="chapter-item expanded "><a href="00_intro/00_installation.html"><strong aria-hidden="true">1.</strong> Installation</a></li><li class="chapter-item expanded "><a href="00_intro/01_introduction.html"><strong aria-hidden="true">2.</strong> Introduction</a></li><li class="chapter-item expanded "><a href="00_intro/02_disclaimer.html"><strong aria-hidden="true">3.</strong> Disclaimer</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="00_intro/03_organization.html"><strong aria-hidden="true">3.1.</strong> Organization</a></li></ol></li><li class="chapter-item expanded "><a href="01_stack/00_stack.html"><strong aria-hidden="true">4.</strong> Stack</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="01_stack/01_structure.html"><strong aria-hidden="true">4.1.</strong> Tech Structure</a></li></ol></li><li class="chapter-item expanded "><a href="02_tables/00_introduction/00_intro.html"><strong aria-hidden="true">5.</strong> Tables</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="02_tables/01_record/00_intro.html"><strong aria-hidden="true">5.1.</strong> Record</a></li><li class="chapter-item expanded "><a href="02_tables/02_views/00_intro.html"><strong aria-hidden="true">5.2.</strong> Views</a></li><li class="chapter-item expanded "><a href="02_tables/03_collection/00_intro.html"><strong aria-hidden="true">5.3.</strong> Collection</a></li><li class="chapter-item expanded "><a href="02_tables/04_collection_view/00_intro.html"><strong aria-hidden="true">5.4.</strong> Collection View</a></li><li class="chapter-item expanded "><a href="02_tables/05_configuration/00_intro.html"><strong aria-hidden="true">5.5.</strong> Configuration</a></li><li class="chapter-item expanded "><a href="02_tables/06_karma/00_karma/00_intro.html"><strong aria-hidden="true">5.6.</strong> Karma</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="02_tables/06_karma/01_condition/00_intro.html"><strong aria-hidden="true">5.6.1.</strong> Karma Condition</a></li><li class="chapter-item expanded "><a href="02_tables/06_karma/02_consequence/00_intro.html"><strong aria-hidden="true">5.6.2.</strong> Karma Consequence</a></li><li class="chapter-item expanded "><a href="02_tables/06_karma/03_frequency/00_intro.html"><strong aria-hidden="true">5.6.3.</strong> Frequency</a></li><li class="chapter-item expanded "><a href="02_tables/06_karma/04_command/00_intro.html"><strong aria-hidden="true">5.6.4.</strong> Command</a></li><li class="chapter-item expanded "><a href="02_tables/06_karma/05_query/00_intro.html"><strong aria-hidden="true">5.6.5.</strong> Query</a></li><li class="chapter-item expanded "><a href="02_tables/06_karma/06_sum/00_intro.html"><strong aria-hidden="true">5.6.6.</strong> Sum</a></li><li class="chapter-item expanded "><a href="02_tables/06_karma/07_examples/00_intro.html"><strong aria-hidden="true">5.6.7.</strong> Examples</a></li></ol></li><li class="chapter-item expanded "><a href="02_tables/07_transfer/00_intro.html"><strong aria-hidden="true">5.7.</strong> Transfer</a></li><li class="chapter-item expanded "><a href="02_tables/08_dna/00_intro.html"><strong aria-hidden="true">5.8.</strong> Dna</a></li><li class="chapter-item expanded "><a href="02_tables/09_history/00_intro.html"><strong aria-hidden="true">5.9.</strong> History</a></li></ol></li><li class="chapter-item expanded "><a href="03_good_practices/00_intro.html"><strong aria-hidden="true">6.</strong> Good Practices</a></li><li class="chapter-item expanded "><a href="04_personal/00_intro.html"><strong aria-hidden="true">7.</strong> Author&#39;s Style</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="04_personal/01_tasks.html"><strong aria-hidden="true">7.1.</strong> Tables</a></li><li class="chapter-item expanded "><a href="04_personal/02_tasks.html"><strong aria-hidden="true">7.2.</strong> Karma</a></li><li class="chapter-item expanded "><a href="04_personal/03_tasks.html"><strong aria-hidden="true">7.3.</strong> Tasks</a></li><li class="chapter-item expanded "><a href="04_personal/04_items.html"><strong aria-hidden="true">7.4.</strong> Items</a></li></ol></li></ol>';
        // Set the current, active page, and reveal it if it's hidden
        let current_page = document.location.href.toString().split("#")[0].split("?")[0];
        if (current_page.endsWith("/")) {
            current_page += "index.html";
        }
        var links = Array.prototype.slice.call(this.querySelectorAll("a"));
        var l = links.length;
        for (var i = 0; i < l; ++i) {
            var link = links[i];
            var href = link.getAttribute("href");
            if (href && !href.startsWith("#") && !/^(?:[a-z+]+:)?\/\//.test(href)) {
                link.href = path_to_root + href;
            }
            // The "index" page is supposed to alias the first chapter in the book.
            if (link.href === current_page || (i === 0 && path_to_root === "" && current_page.endsWith("/index.html"))) {
                link.classList.add("active");
                var parent = link.parentElement;
                if (parent && parent.classList.contains("chapter-item")) {
                    parent.classList.add("expanded");
                }
                while (parent) {
                    if (parent.tagName === "LI" && parent.previousElementSibling) {
                        if (parent.previousElementSibling.classList.contains("chapter-item")) {
                            parent.previousElementSibling.classList.add("expanded");
                        }
                    }
                    parent = parent.parentElement;
                }
            }
        }
        // Track and set sidebar scroll position
        this.addEventListener('click', function(e) {
            if (e.target.tagName === 'A') {
                sessionStorage.setItem('sidebar-scroll', this.scrollTop);
            }
        }, { passive: true });
        var sidebarScrollTop = sessionStorage.getItem('sidebar-scroll');
        sessionStorage.removeItem('sidebar-scroll');
        if (sidebarScrollTop) {
            // preserve sidebar scroll position when navigating via links within sidebar
            this.scrollTop = sidebarScrollTop;
        } else {
            // scroll sidebar to current active section when navigating via "next/previous chapter" buttons
            var activeSection = document.querySelector('#sidebar .active');
            if (activeSection) {
                activeSection.scrollIntoView({ block: 'center' });
            }
        }
        // Toggle buttons
        var sidebarAnchorToggles = document.querySelectorAll('#sidebar a.toggle');
        function toggleSection(ev) {
            ev.currentTarget.parentElement.classList.toggle('expanded');
        }
        Array.from(sidebarAnchorToggles).forEach(function (el) {
            el.addEventListener('click', toggleSection);
        });
    }
}
window.customElements.define("mdbook-sidebar-scrollbox", MDBookSidebarScrollbox);
