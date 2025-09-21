rust + axum + redis + mysql for web background server.
It's still in studing & working.



insert into t2 (id, name, age,sex) values(1, 'zcg', 10, 'f');




create table t2 (  
 id int,  
 name varchar(20),  
 age int,  
 sex varchar(20)
);


   let result = sqlx::query("INSERT INTO users (name) VALUES ($1)")
        .bind(name)
        .execute(pool)
        .await?;




cargo install cargo-local-registry

mkdir local-registry
cd    local-registry

cargo local-registry --sync ..\rweb\Cargo.lock   .\


将整个 local-registry 目录通过 U 盘、内部网络或其他方式，复制到你需要进行离线构建的机器上。



[registries]
my-offline-index = { index = "file:///absolute/path/to/my-local-registry" }

# 【可选但强烈推荐】将默认注册源设置为你的本地源
# 这样就不需要在每个依赖项后手动添加 `registry = "my-offline-index"`
[source]
# 首先，替换默认的 crates.io 源，指向你的本地源
[source.crates-io]
replace-with = "my-offline-index"

# 然后，定义你的本地源
[source.my-offline-index]
registry = "file:///absolute/path/to/my-local-registry"

# 如果你还需要官方的 crates.io 作为后备（在混合网络环境下），可以取消注释并这样设置
# [source.crates-io]
# replace-with = "my-offline-index" # 优先使用本地
#
# [source.my-offline-index]
# registry = "file:///absolute/path/to/my-local-registry"
#
# [source.original-crates-io] # 定义一个备用源，指向真正的 crates.io
# registry = "https://github.com/rust-lang/crates.io-index"
-----------------------------------------------------------------------------------------或者-----------------

add this to your .cargo/config somewhere:

    [source.crates-io]
    registry = 'sparse+https://index.crates.io/'
    replace-with = 'local-registry'

    [source.local-registry]
    local-registry = 'D:\workspace\my_register\.\'




第 4 步：在离线环境中进行构建
cd my_offline_project
cargo build