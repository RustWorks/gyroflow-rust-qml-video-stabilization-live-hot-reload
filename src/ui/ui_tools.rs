
use qmetaobject::*;
use cpp::*;
use std::cell::RefCell;
use crate::controller::Controller;
use crate::util;

cpp! {{
    #include <QTranslator>
}}

#[derive(Default, QObject)]
pub struct UITools { 
    base: qt_base_class!(trait QObject), 
    set_theme: qt_method!(fn(&self, theme: String)),
    set_language: qt_method!(fn(&self, lang_id: QString)),
    init_calibrator: qt_method!(fn(&mut self)),
    set_icon: qt_method!(fn(&self, wnd: QJSValue)),

    language_changed: qt_signal!(),

    calibrator_ctl: Option<RefCell<Controller>>,

    pub engine_ptr: Option<*mut QmlEngine>
}
impl UITools {
    pub fn set_language(&self, lang_id: QString) {
        if let Some(engine) = self.engine_ptr {
            let engine = unsafe { &mut *(engine) };
            let engine_ptr = engine.cpp_ptr();
            cpp!(unsafe [engine_ptr as "QQmlEngine *", lang_id as "QString"] {
                static QTranslator translator;
                QCoreApplication::removeTranslator(&translator);

                if (lang_id != "en") {
                    if (translator.load(":/resources/translations/" + lang_id + ".qm")) {
                        QCoreApplication::installTranslator(&translator);
                    }
                }

                engine_ptr->retranslate();
            });
            self.language_changed();
        }
    }

    pub fn set_theme(&self, theme: String) {
        if let Some(engine) = self.engine_ptr {
            let engine = unsafe { &mut *(engine) };
        
            cpp!(unsafe [] { auto f = QGuiApplication::font(); f.setFamily("Arial"); QGuiApplication::setFont(f); });
            engine.set_property("styleFont".into(), QString::from("Arial").into());

            match theme.as_str() {
                "dark" => {
                    engine.set_property("style"                 .into(), QString::from("dark").into());
                    engine.set_property("styleBackground"       .into(), QString::from("#272727").into());
                    engine.set_property("styleBackground2"      .into(), QString::from("#202020").into());
                    engine.set_property("styleButtonColor"      .into(), QString::from("#2d2d2d").into());
                    engine.set_property("styleTextColor"        .into(), QString::from("#ffffff").into());
                    engine.set_property("styleAccentColor"      .into(), QString::from("#76baed").into());
                    engine.set_property("styleVideoBorderColor" .into(), QString::from("#313131").into());
                    engine.set_property("styleTextColorOnAccent".into(), QString::from("#000000").into());
                    engine.set_property("styleHrColor"          .into(), QString::from("#323232").into());
                    engine.set_property("stylePopupBorder"      .into(), QString::from("#141414").into());
                    engine.set_property("styleSliderHandle"     .into(), QString::from("#454545").into());
                    engine.set_property("styleSliderBackground" .into(), QString::from("#9a9a9a").into());
                    engine.set_property("styleHighlightColor"   .into(), QString::from("#10ffffff").into());
                },
                "light" => {
                    engine.set_property("style"                 .into(), QString::from("light").into());
                    engine.set_property("styleBackground"       .into(), QString::from("#f9f9f9").into());
                    engine.set_property("styleBackground2"      .into(), QString::from("#f3f3f3").into());
                    engine.set_property("styleButtonColor"      .into(), QString::from("#fbfbfb").into());
                    engine.set_property("styleTextColor"        .into(), QString::from("#111111").into());
                    engine.set_property("styleAccentColor"      .into(), QString::from("#116cad").into());
                    engine.set_property("styleVideoBorderColor" .into(), QString::from("#d5d5d5").into());
                    engine.set_property("styleTextColorOnAccent".into(), QString::from("#ffffff").into());
                    engine.set_property("styleHrColor"          .into(), QString::from("#e5e5e5").into());
                    engine.set_property("stylePopupBorder"      .into(), QString::from("#d5d5d5").into());
                    engine.set_property("styleSliderHandle"     .into(), QString::from("#c2c2c2").into());
                    engine.set_property("styleSliderBackground" .into(), QString::from("#cdcdcd").into());
                    engine.set_property("styleHighlightColor"   .into(), QString::from("#10000000").into());
                },
                _ => { }
            }
        }
    }

    pub fn set_icon(&self, wnd: QJSValue) {
        cpp!(unsafe [wnd as "QJSValue"] {
            auto obj = qobject_cast<QQuickWindow *>(wnd.toQObject());
            obj->setIcon(QIcon(":/resources/icon.png"));
        });
    }

    pub fn init_calibrator(&mut self) {
        //if self.calibrator_ctl.is_none() {
            self.calibrator_ctl = Some(RefCell::new(Controller::new()));

            let calib_ctl = self.calibrator_ctl.as_ref().unwrap();
            calib_ctl.borrow().init_calibrator();
            let calib_ctlpinned = unsafe { QObjectPinned::new(calib_ctl) };
    
            if let Some(engine) = self.engine_ptr {
                let engine = unsafe { &mut *(engine) };
                engine.set_object_property("calib_controller".into(), calib_ctlpinned);

                calib_ctl.borrow_mut().stabilizer.params.write().framebuffer_inverted = util::is_opengl();
            }
        //}
    }
}
