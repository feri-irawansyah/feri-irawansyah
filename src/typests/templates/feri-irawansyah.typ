// ── Page setup ────────────────────────────────────────────────────────────────
#set page(
  paper: "a4",
  margin: (top: 1cm, bottom: 1cm, left: 1.3cm, right: 1.3cm),
)
#set text(font: "DejaVu Sans", size: 9pt, fill: luma(30))
#set par(leading: 0.52em)

// ── Colours ───────────────────────────────────────────────────────────────────
#let accent = rgb("#0d9488")
#let muted = luma(115)
#let line-col = luma(220)
#let tag-bg = luma(243)

// ── Icon helper ───────────────────────────────────────────────────────────────
#let icon(file) = box(
  baseline: 18%,
  image(file, height: 0.78em),
)

// ── Section header with accent left bar ───────────────────────────────────────
#let section(title) = {
  v(0.5em)
  grid(
    columns: (2.5pt, auto),
    column-gutter: 0.45em,
    align: horizon,
    rect(fill: accent, radius: 0.8pt, width: 100%, height: 0.9em),
    text(weight: "bold", size: 8.5pt, fill: accent, upper(title)),
  )
  v(0.08em)
  line(length: 100%, stroke: 0.5pt + line-col)
  v(0.22em)
}

// ── Experience entry ──────────────────────────────────────────────────────────
#let entry(title, subtitle, period, location, bullets) = {
  grid(
    columns: (1fr, auto),
    text(weight: "bold", size: 9pt, title), text(size: 8pt, fill: muted, period),
  )
  v(0.03em)
  text(size: 8pt, fill: muted, location)
  v(0.05em)
  text(size: 8pt, fill: accent, subtitle)
  if bullets.len() > 0 {
    v(0.12em)
    for b in bullets {
      [#text(fill: accent)[▸] #text(size: 8.5pt, b) \ ]
    }
  }
  v(0.28em)
}

// ── Sub-role under a company ──────────────────────────────────────────────────
#let sub-entry(role, period, bullets) = {
  grid(
    columns: (1fr, auto),
    text(size: 8pt, fill: accent, role), text(size: 8pt, fill: muted, period),
  )
  v(0.1em)
  for b in bullets [
    #text(fill: accent)[▸] #text(size: 8.5pt, b) \
  ]
  v(0.2em)
}

// ── Skill row with tag chips ──────────────────────────────────────────────────
#let tag(t) = box(
  fill: tag-bg,
  inset: (x: 4pt, y: 2pt),
  radius: 2.5pt,
  text(size: 7.5pt, t),
)

#let skill-row(label, items) = {
  grid(
    columns: (1.9cm, 1fr),
    column-gutter: 0em,
    row-gutter: 0em,
    align: (left + horizon, left + horizon),
    text(weight: "bold", size: 7.5pt, fill: muted, label + ":"), [#items.map(tag).join(h(0.28em))],
  )
  v(0.16em)
}

// ── Contact item ──────────────────────────────────────────────────────────────
#let sep = h(0.55em) + text(fill: luma(195))[|] + h(0.55em)

// ══════════════════════════════════════════════════════════════════════════════
// HEADER
// ══════════════════════════════════════════════════════════════════════════════
#grid(
  columns: (auto, 1fr),
  gutter: 1.2em,
  align: horizon,

  box(
    clip: true,
    radius: 5pt,
    width: 2.4cm,
    height: 2.9cm,
    image("img/feri-irawansyah.jpg", width: 100%, height: 100%, fit: "cover"),
  ),

  [
    #text(size: 19pt, weight: "bold", "Feri Irawansyah") \
    #v(0.06em)
    #text(size: 9.5pt, fill: accent, weight: "semibold", "Backend Engineer · Web Developer · AI Engineer") \
    #v(0.45em)
    #text(size: 8.5pt, fill: muted)[
      #icon("svg/envelope.svg") #h(0.2em) ir15y4hh\@gmail.com
      #sep
      #icon("svg/telephone-inbound.svg") #h(0.2em) +62 823-2344-3535
      #sep
      #icon("svg/github.svg") #h(0.2em) github.com/feri-irawansyah \
      #v(0.28em)
      #icon("svg/geo-fill.svg") #h(0.2em) DKI Jakarta, Indonesia
      #sep
      #icon("svg/globe-check.svg") #h(0.2em) feri-irawansyah.my.id
    ]
  ],
)

#v(0.3em)
#line(length: 100%, stroke: 1pt + line-col)
#v(0.3em)

// ══════════════════════════════════════════════════════════════════════════════
// BODY
// ══════════════════════════════════════════════════════════════════════════════
#grid(
  columns: (6fr, 4fr),
  gutter: 1.1em,

  // ── LEFT COLUMN ─────────────────────────────────────────────────────────────
  [
    #section("Profile")
    #text(
      size: 9pt,
      "Backend and web engineer with 4+ years of hands-on experience delivering "
        + "production applications across fintech and enterprise sectors. Skilled in "
        + "building REST APIs, admin dashboards, and back-office systems with .NET Core, "
        + "SvelteKit, Angular, and React — increasingly adopting Rust for performance-critical "
        + "services. Experienced in data science and machine learning workflows using Python.",
    )
    #v(0.32em)
    #grid(
      columns: (2.5cm, 1fr, 2.5cm, 1fr),
      column-gutter: 0.5em,
      row-gutter: 0.45em,
      text(size: 8pt, fill: muted, "Date of Birth:"),
      text(size: 8pt, "9 June 2000"),
      text(size: 8pt, fill: muted, "Nationality:"),
      text(size: 8pt, "Indonesian"),
    )
    #v(0.2em)
    #grid(
      columns: (2.5cm, 1fr),
      column-gutter: 0.5em,
      row-gutter: 0.45em,
      text(size: 8pt, fill: muted, "Location:"), text(size: 8pt, "DKI Jakarta"),
      text(size: 8pt, fill: muted, "LinkedIn:"), text(size: 8pt, "linkedin.com/in/feri-irawansyah"),
    )
    #v(0.15em)

    #section("Experience")

    #grid(
      columns: (1fr, auto),
      text(weight: "bold", size: 9pt, "PT Micropiranti Computer"), text(size: 8pt, fill: muted, "Sep 2022 – Apr 2026"),
    )
    #v(0.03em)
    #text(size: 8pt, fill: muted, "Jakarta Selatan · Fulltime · Hybrid")
    #v(0.2em)

    #sub-entry("Web Back Office", "Mar 2023 – Apr 2026", (
      "Rewrote legacy VB.NET back-office application to ASP.NET Core.",
      "Tuned database performance using Stored Procedures as business layer.",
      "Built a full REST API with ASP.NET Core for internal and client-side consumers.",
      "Developed admin panel UI for the back-office system using AngularJS.",
    ))

    #sub-entry("Web Developer", "Sep 2022 – Mar 2024", (
      "Built an RDN registration web app using ASP.NET Framework MVC and AngularJS.",
      "Managed SQL Server schemas and performed custom CSS styling per client spec.",
      "Developed admin dashboard with ReactJS; mentored junior team members on React.",
    ))
    #v(0.1em)

    #entry(
      "PT Zona Akselerasi Pendidikan",
      "Accelerated Machine Learning",
      "Feb – Jul 2022",
      "Jakarta, Indonesia · Internship",
      (
        "Analysed SQL datasets using Python to surface actionable insights.",
        "Performed data storytelling to frame business use-cases from raw data.",
        "Built visualisations with Python and Tableau to depict data content clearly.",
        "Cleaned and pre-processed datasets to make them ready for model training.",
        "Trained ML models using algorithms matched to each identified use-case.",
      ),
    )

    #entry(
      "PT Orbit Ventura Indonesia",
      "AI Engineer",
      "Aug 2021 – Jan 2022",
      "Jakarta Selatan · Internship · Online",
      (
        "Studied AI fundamentals and built classification models using Python.",
        "Performed exploratory data analysis and statistical case studies.",
        "Deployed ML models to web using Streamlit and Flask.",
      ),
    )
  ],

  // ── RIGHT COLUMN ────────────────────────────────────────────────────────────
  [
    #section("Technical Skills")

    #skill-row("Languages", ("C#", "Rust", "Python", "SQL", "TypeScript"))
    #skill-row("Backend", ("ASP.NET Core", "REST API", "Actix Web", "Bun"))
    #skill-row("Frontend", ("SvelteKit", "Angular", "ReactJS", "Tailwind CSS"))
    #skill-row("Database", ("SQL Server", "PostgreSQL", "SQLx", "Dapper"))
    #skill-row("AI / ML", ("Machine Learning", "Deep Learning", "Tableau"))
    #skill-row("DevOps", ("Git", "Docker", "Linux", "IIS", "Nginx"))

    #section("Education")
    #entry(
      "Universitas Pamulang",
      "Sarjana Akuntansi (S.Ak)",
      "2019 – 2023",
      "South Tangerang, Indonesia",
      (),
    )

    #section("Certifications")
    #entry(
      "Orbit Future Academy",
      "AI Engineer",
      "Aug 2021 – Jan 2022",
      "Online",
      (),
    )
    #entry(
      "Accelerated ML",
      "PT Zona Akselerasi Pendidikan",
      "Feb – Jul 2022",
      "Online",
      (),
    )
    #entry(
      "IDCamp 2022",
      "Machine Learning — PT Indosat Tbk",
      "May – Sep 2022",
      "Online",
      (),
    )

    #section("Soft Skills")
    #text(
      size: 8.5pt,
      "Self-directed learning · Analytical thinking · "
        + "Technical mentoring · Remote collaboration · "
        + "Performance mindset",
    )
    #v(0.3em)
  ],
)
