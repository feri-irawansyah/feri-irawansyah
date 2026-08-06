-- Add up migration script here
INSERT INTO
    skills (
        title,
        url_docs,
        image_src,
        progress,
        star,
        last_update,
        description,
        experience,
        tech_category
    )
VALUES (
        'Sass',
        'https://sass-lang.com',
        'https://vjwknqthtunirowwtrvj.supabase.co/storage/v1/object/public/feri-irawansyah.my.id/assets/svg/skills/sass.svg',
        80,
        4,
        '2025-07-22 14:59:33.708676+07',
        'Css preprosesor paling enak dipakai',
        3,
        'programming'
    ),
    (
        'Bootstrap',
        'https://getbootstrap.com',
        'https://vjwknqthtunirowwtrvj.supabase.co/storage/v1/object/public/feri-irawansyah.my.id/assets/svg/skills/bootstrap.svg',
        95,
        5,
        '2025-07-22 14:59:33.708676+07',
        'Css framework paling cocok untuk bikin design cepet',
        5,
        'techstack'
    ),
    (
        'Handlebars',
        'https://handlebarsjs.com',
        'https://vjwknqthtunirowwtrvj.supabase.co/storage/v1/object/public/feri-irawansyah.my.id/assets/svg/skills/hbs.svg',
        80,
        3,
        '2025-07-22 14:59:33.708676+07',
        'Templating alternative untuk email template yang ringan ',
        2,
        'programming'
    ),
    (
        'Leptos',
        'https://leptos.dev',
        'https://vjwknqthtunirowwtrvj.supabase.co/storage/v1/object/public/feri-irawansyah.my.id/assets/img/skills/leptos.png',
        80,
        5,
        '2025-07-11 00:19:03.042873+07',
        'Framework fullstack favorit gue',
        1,
        'techstack'
    ),
    (
        'Rust',
        'https://www.rust-lang.org/',
        'https://vjwknqthtunirowwtrvj.supabase.co/storage/v1/object/public/feri-irawansyah.my.id/assets/svg/skills/rust.svg',
        95,
        5,
        '2025-07-03 05:04:12.930274+07',
        'Rush adalah bahasa pemrograman favorit gue',
        3,
        'programming'
    ),
    (
        'Ubuntu',
        'https://ubuntu.com',
        'https://vjwknqthtunirowwtrvj.supabase.co/storage/v1/object/public/feri-irawansyah.my.id/assets/svg/skills/ubuntu.svg',
        75,
        5,
        '2025-07-11 00:59:35.569007+07',
        'Distro paling cakep',
        2,
        'devops'
    ),
    (
        'Nginx',
        'https://nginx.org',
        'https://vjwknqthtunirowwtrvj.supabase.co/storage/v1/object/public/feri-irawansyah.my.id/assets/svg/skills/nginx.svg',
        95,
        5,
        '2025-07-14 03:55:45.616591+07',
        'Web server ringan dan kenceng',
        4,
        'devops'
    ),
    (
        'Javascript',
        'https://www.w3schools.com/js',
        'https://vjwknqthtunirowwtrvj.supabase.co/storage/v1/object/public/feri-irawansyah.my.id/assets/svg/skills/js.svg',
        95,
        5,
        '2025-07-14 03:58:45.813227+07',
        'Satu-satunya bahasa kesayangan DOM',
        3,
        'programming'
    ),
    (
        'Dart',
        'https://dart.dev',
        'https://vjwknqthtunirowwtrvj.supabase.co/storage/v1/object/public/feri-irawansyah.my.id/assets/svg/skills/dart.svg',
        60,
        3,
        '2025-07-14 04:42:28.108888+07',
        'Khusus untuk mobile dev',
        2,
        'programming'
    ),
    (
        'Html',
        'https://www.w3schools.com/html/default.asp',
        'https://vjwknqthtunirowwtrvj.supabase.co/storage/v1/object/public/feri-irawansyah.my.id/assets/svg/skills/html.svg',
        100,
        5,
        '2025-07-14 04:00:16.151503+07',
        'Templating dasar umat manusia paling dasar',
        5,
        'programming'
    );

INSERT INTO
    skills (
        title,
        url_docs,
        image_src,
        progress,
        star,
        last_update,
        description,
        experience,
        tech_category
    )
VALUES (
        'Css',
        'https://www.w3schools.com/css/default.asp',
        'https://vjwknqthtunirowwtrvj.supabase.co/storage/v1/object/public/feri-irawansyah.my.id/assets/svg/skills/css.svg',
        95,
        5,
        '2025-07-14 04:04:31.817142+07',
        'Leluhur stylesheet untuk website',
        5,
        'programming'
    ),
    (
        'C#',
        'https://learn.microsoft.com/en-us/dotnet/csharp',
        'https://vjwknqthtunirowwtrvj.supabase.co/storage/v1/object/public/feri-irawansyah.my.id/assets/svg/skills/cs.svg',
        85,
        5,
        '2025-07-14 04:05:53.529188+07',
        'Bahasa kesayangan microsoft',
        3,
        'programming'
    ),
    (
        'Typescript',
        'https://www.typescriptlang.org',
        'https://vjwknqthtunirowwtrvj.supabase.co/storage/v1/object/public/feri-irawansyah.my.id/assets/svg/skills/ts.svg',
        90,
        5,
        '2025-07-14 04:07:26.8184+07',
        'Javascript yang menggunakan jas',
        3,
        'programming'
    ),
    (
        'Python',
        'https://www.python.org',
        'https://vjwknqthtunirowwtrvj.supabase.co/storage/v1/object/public/feri-irawansyah.my.id/assets/svg/skills/python.svg',
        90,
        5,
        '2025-07-14 04:10:22.347547+07',
        'Bahasa pemrograman bestienya AI dan ML',
        6,
        'programming'
    ),
    (
        'SQL',
        'https://www.w3schools.com/sql/default.asp',
        'https://vjwknqthtunirowwtrvj.supabase.co/storage/v1/object/public/feri-irawansyah.my.id/assets/img/skills/sql.jpg',
        90,
        5,
        '2025-07-14 04:17:52.789371+07',
        'Bahasa untuk negoisasi sama data',
        5,
        'programming'
    ),
    (
        'PHP',
        'https://www.php.net',
        'https://vjwknqthtunirowwtrvj.supabase.co/storage/v1/object/public/feri-irawansyah.my.id/assets/svg/skills/php.svg',
        80,
        3,
        '2025-07-14 04:44:33.05417+07',
        'Bahasa realita dengan kehidupan programmer',
        4,
        'programming'
    ),
    (
        'Actix Web',
        'https://actix.rs',
        'https://vjwknqthtunirowwtrvj.supabase.co/storage/v1/object/public/feri-irawansyah.my.id/assets/svg/skills/actix.svg',
        90,
        5,
        '2025-07-11 00:17:39.881399+07',
        'Framework web backend favorit gue',
        2,
        'techstack'
    ),
    (
        'Svelte',
        'https://svelte.dev',
        'https://vjwknqthtunirowwtrvj.supabase.co/storage/v1/object/public/feri-irawansyah.my.id/assets/svg/skills/svelte.svg',
        95,
        5,
        '2025-07-20 22:47:29.333008+07',
        'Framework frontend favorit gue',
        2,
        'techstack'
    ),
    (
        '.Net Core',
        'https://dotnet.microsoft.com',
        'https://vjwknqthtunirowwtrvj.supabase.co/storage/v1/object/public/feri-irawansyah.my.id/assets/svg/skills/netcore.svg',
        80,
        4,
        '2025-07-20 22:54:01.892452+07',
        'Framework backend enterprice',
        3,
        'techstack'
    ),
    (
        'React',
        'https://react.dev',
        'https://vjwknqthtunirowwtrvj.supabase.co/storage/v1/object/public/feri-irawansyah.my.id/assets/svg/skills/react.svg',
        90,
        4,
        '2025-07-20 22:54:01.892452+07',
        'Framework frontend paling populer',
        4,
        'techstack'
    );

INSERT INTO
    skills (
        title,
        url_docs,
        image_src,
        progress,
        star,
        last_update,
        description,
        experience,
        tech_category
    )
VALUES (
        'Tauri',
        'https://v2.tauri.app',
        'https://vjwknqthtunirowwtrvj.supabase.co/storage/v1/object/public/feri-irawansyah.my.id/assets/svg/skills/tauri.svg',
        85,
        5,
        '2025-07-20 23:01:16.563678+07',
        'Alternative desktop app favorit gue',
        1,
        'techstack'
    ),
    (
        'Bun',
        'https://bun.com',
        'https://vjwknqthtunirowwtrvj.supabase.co/storage/v1/object/public/feri-irawansyah.my.id/assets/svg/skills/bun.svg',
        90,
        5,
        '2025-07-20 23:01:16.563678+07',
        'Framework web api ringan dan cepat',
        2,
        'techstack'
    ),
    (
        'PostgreSQL',
        'https://www.postgresql.org',
        'https://vjwknqthtunirowwtrvj.supabase.co/storage/v1/object/public/feri-irawansyah.my.id/assets/svg/skills/pgsql.svg',
        95,
        5,
        '2025-07-20 23:17:54.519409+07',
        'Database relational untuk umat free ',
        5,
        'techstack'
    ),
    (
        'SQL Server',
        'https://learn.microsoft.com/en-us/sql/?view=sql-server-ver17',
        'https://vjwknqthtunirowwtrvj.supabase.co/storage/v1/object/public/feri-irawansyah.my.id/assets/svg/skills/mssql.svg',
        90,
        5,
        '2025-07-20 23:17:54.519409+07',
        'Database enterprice untuk stack microsoft',
        3,
        'techstack'
    ),
    (
        'Docker',
        'https://docs.docker.com',
        'https://vjwknqthtunirowwtrvj.supabase.co/storage/v1/object/public/feri-irawansyah.my.id/assets/svg/skills/docker.svg',
        70,
        5,
        '2025-07-20 23:31:42.921535+07',
        'Container paling mudah dan populer ',
        2,
        'devops'
    ),
    (
        'Redis',
        'https://redis.io',
        'https://vjwknqthtunirowwtrvj.supabase.co/storage/v1/object/public/feri-irawansyah.my.id/assets/svg/skills/redis.svg',
        75,
        5,
        '2025-07-20 23:31:42.921535+07',
        'Database key value untuk cache ',
        2,
        'devops'
    ),
    (
        'Kubernetes',
        'https://kubernetes.io',
        'https://vjwknqthtunirowwtrvj.supabase.co/storage/v1/object/public/feri-irawansyah.my.id/assets/svg/skills/kubernetes.svg',
        60,
        5,
        '2025-07-20 23:31:42.921535+07',
        'Management aplikasi agar selalu on',
        1,
        'devops'
    ),
    (
        'Linux',
        'https://www.linux.org',
        'https://vjwknqthtunirowwtrvj.supabase.co/storage/v1/object/public/feri-irawansyah.my.id/assets/svg/skills/linux.svg',
        70,
        5,
        '2025-07-20 23:31:42.921535+07',
        'OS paling enak buat build dan deployment',
        2,
        'devops'
    ),
    (
        'Windows',
        'https://www.microsoft.com/en-us/windows/get-windows-11',
        'https://vjwknqthtunirowwtrvj.supabase.co/storage/v1/object/public/feri-irawansyah.my.id/assets/svg/skills/windows.svg',
        100,
        5,
        '2025-07-20 23:31:42.921535+07',
        'OS buat kegiatan sehari hari',
        6,
        'devops'
    ),
    (
        'Git',
        'https://git-scm.com',
        'https://vjwknqthtunirowwtrvj.supabase.co/storage/v1/object/public/feri-irawansyah.my.id/assets/svg/skills/git.svg',
        100,
        5,
        '2025-07-20 23:52:41.749986+07',
        'Version control system umat manusia',
        6,
        'devops'
    );

INSERT INTO
    skills (
        title,
        url_docs,
        image_src,
        progress,
        star,
        last_update,
        description,
        experience,
        tech_category
    )
VALUES (
        'Rabbit MQ',
        'https://www.rabbitmq.com',
        'https://vjwknqthtunirowwtrvj.supabase.co/storage/v1/object/public/feri-irawansyah.my.id/assets/svg/skills/rabbitmq.svg',
        0,
        5,
        '2025-07-20 23:52:41.749986+07',
        'Tools message broker favorit gue',
        1,
        'devops'
    ),
    (
        'Vite',
        'https://vite.dev',
        'https://vjwknqthtunirowwtrvj.supabase.co/storage/v1/object/public/feri-irawansyah.my.id/assets/svg/skills/vite.svg',
        90,
        5,
        '2025-07-21 01:03:28.823172+07',
        'Frontend build tool paling gue suka',
        4,
        'techstack'
    ),
    (
        'Swagger',
        'https://swagger.io',
        'https://vjwknqthtunirowwtrvj.supabase.co/storage/v1/object/public/feri-irawansyah.my.id/assets/svg/skills/swagger.svg',
        100,
        5,
        '2025-07-21 14:51:59.602587+07',
        'Fitur api docs paling powerfull menurut gue',
        3,
        'programming'
    ),
    (
        'Shuttle',
        'https://www.shuttle.dev',
        'https://vjwknqthtunirowwtrvj.supabase.co/storage/v1/object/public/feri-irawansyah.my.id/assets/img/skills/shuttle.webp',
        70,
        5,
        '2025-07-21 15:00:07.816977+07',
        'Cloud platform free alternative untuk Rust dev',
        1,
        'devops'
    ),
    (
        'Rust Acean',
        'https://github.com/nrc/rustaceans.org',
        'https://vjwknqthtunirowwtrvj.supabase.co/storage/v1/object/public/feri-irawansyah.my.id/assets/img/skills/rustacean.webp',
        50,
        5,
        '2025-07-21 15:07:26.38959+07',
        'Komunitas pecinta kepiting yang susah di compile',
        2,
        'programming'
    ),
    (
        'JWT',
        'https://jwt.io',
        'https://vjwknqthtunirowwtrvj.supabase.co/storage/v1/object/public/feri-irawansyah.my.id/assets/svg/skills/jwt.svg',
        90,
        5,
        '2025-07-22 14:59:33.708676+07',
        'Library paling bagus untuk tokenization',
        3,
        'techstack'
    ),
    (
        'Cloudflare',
        'https://www.cloudflare.com',
        'https://vjwknqthtunirowwtrvj.supabase.co/storage/v1/object/public/feri-irawansyah.my.id/assets/svg/skills/cloudflare.svg',
        80,
        4,
        '2025-07-27 09:13:21.009503+07',
        'Cloud platform yang free tiernya serasa enterprice',
        4,
        'devops'
    ),
    (
        'IIS',
        'https://www.iis.net',
        'https://vjwknqthtunirowwtrvj.supabase.co/storage/v1/object/public/feri-irawansyah.my.id/assets/img/skills/iis.webp',
        70,
        3,
        '2025-10-04 19:56:54.12238+07',
        'Web Server untuk Windows only',
        3,
        'devops'
    ),
    (
        'GraphQL',
        'https://graphql.org',
        'https://vjwknqthtunirowwtrvj.supabase.co/storage/v1/object/public/feri-irawansyah.my.id/assets/svg/skills/graphql.svg',
        70,
        4,
        '2025-10-13 00:57:59.608628+07',
        'Alternative Simple REST API',
        3,
        'programming'
    ),
    (
        'Alibaba',
        'https://www.alibabacloud.com',
        'https://vjwknqthtunirowwtrvj.supabase.co/storage/v1/object/public/feri-irawansyah.my.id/assets/svg/skills/alibaba.svg',
        70,
        3,
        '2025-07-27 09:01:16.533619+07',
        'Cloud provider lumayan bisa dipelajari dan banyak beasiswa',
        1,
        'devops'
    );

INSERT INTO
    skills (
        title,
        url_docs,
        image_src,
        progress,
        star,
        last_update,
        description,
        experience,
        tech_category
    )
VALUES (
        'Tailwindcss',
        'https://tailwindcss.com',
        'https://vjwknqthtunirowwtrvj.supabase.co/storage/v1/object/public/feri-irawansyah.my.id/assets/img/skills/tailwindcss.webp',
        80,
        4,
        '2025-10-04 19:56:54.12238+07',
        'Css framework yang sangat populer',
        4,
        'techstack'
    ),
    (
        'Wasm',
        'https://webassembly.org',
        'https://vjwknqthtunirowwtrvj.supabase.co/storage/v1/object/public/feri-irawansyah.my.id/assets/svg/skills/wasm.svg',
        90,
        5,
        '2025-10-13 00:57:59.608628+07',
        'Native performance in web application',
        5,
        'programming'
    );
