// SPDX-FileCopyrightText: 2026 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: sammy <sam@bombus.cloud>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qstring.h");
        type QString = crate::QString;

        include!("cxx-qt-lib/sql/qsqldatabase.h");
        /// The QSqlDatabase class handles a connection to a database.
        ///
        /// Qt docs: https://doc.qt.io/qt-6/qsqldatabase.html
        type QSqlDatabase;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        #[doc(hidden)]
        #[rust_name = "qsqldatabase_new"]
        fn qsqldatabaseNew(args: &QVector_QByteArray) -> UniquePtr<QSqlDatabase>;
    }
}
