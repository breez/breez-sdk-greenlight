import Foundation

@objc(BreezSDK)
class BreezSDK: RCTEventEmitter {
    static let TAG: String = "BreezSDK"
    
    private var breezServices: BlockingBreezServices!
    
    @objc
    override static func moduleName() -> String! {
        TAG
    }
    
    override func supportedEvents() -> [String]! {
        return [BreezSDKListener.emitterName, BreezSDKLogStream.emitterName]
    }
    
    @objc
    override static func requiresMainQueueSetup() -> Bool {
        return false
    }
    
    @objc(mnemonicToSeed:resolver:rejecter:)
    func mnemonicToSeed(_ phrase: String, resolver resolve: @escaping RCTPromiseResolveBlock, rejecter reject: @escaping RCTPromiseRejectBlock) -> Void {
        do {
            let seed = try breez_sdk.mnemonicToSeed(phrase: phrase)
            
            resolve(seed)
        } catch let err {
            reject(BreezSDK.TAG, "Error calling mnemonicToSeed", err)
        }
    }
    
    @objc(parseInput:resolver:rejecter:)
    func parseInput(_ input: String, resolver resolve: @escaping RCTPromiseResolveBlock, rejecter reject: @escaping RCTPromiseRejectBlock) -> Void {
        do {
            let inputType = try breez_sdk.parseInput(s: input)
            
            resolve(BreezSDKMapper.dictionaryOf(inputType: inputType))
        } catch let err {
            reject(BreezSDK.TAG, "Error calling parseInput", err)
        }
    }
    
    @objc(parseInvoice:resolver:rejecter:)
    func parseInvoice(_ invoice: String, resolver resolve: @escaping RCTPromiseResolveBlock, rejecter reject: @escaping RCTPromiseRejectBlock) -> Void {
        do {
            let lnInvoice = try breez_sdk.parseInvoice(invoice: invoice)
            
            resolve(BreezSDKMapper.dictionaryOf(lnInvoice: lnInvoice))
        } catch let err {
            reject(BreezSDK.TAG, "Error calling parseInvoice", err)
        }
    }
    
    @objc(registerNode:seed:resolver:rejecter:)
    func registerNode(_ network:String, seed:[UInt8], resolver resolve: @escaping RCTPromiseResolveBlock, rejecter reject: @escaping RCTPromiseRejectBlock) -> Void {
        do {
            let greenlightCredentials = try breez_sdk.registerNode(network: BreezSDKMapper.asNetwork(network: network), seed: seed)
            
            resolve(BreezSDKMapper.dictionaryOf(greenlightCredentials: greenlightCredentials))
        } catch let err {
            reject(BreezSDK.TAG, "Error calling registerNode", err)
        }
    }
    
    @objc(recoverNode:seed:resolver:rejecter:)
    func recoverNode(_ network:String, seed:[UInt8], resolver resolve: @escaping RCTPromiseResolveBlock, rejecter reject: @escaping RCTPromiseRejectBlock) -> Void {
        do {
            let greenlightCredentials = try breez_sdk.recoverNode(network: BreezSDKMapper.asNetwork(network: network), seed: seed)
            
            resolve(BreezSDKMapper.dictionaryOf(greenlightCredentials: greenlightCredentials))
        } catch let err {
            reject(BreezSDK.TAG, "Error calling recoverNode", err)
        }
    }
    
    @objc(startLogStream:rejecter:)
    func startLogStream(_ resolve: @escaping RCTPromiseResolveBlock, rejecter reject: @escaping RCTPromiseRejectBlock) -> Void {
        do {
            try breez_sdk.setLogStream(logStream: BreezSDKLogStream(emitter: self))
            
            resolve(["status": "ok"])
        } catch let err {
            reject(BreezSDK.TAG, "Error calling setLogStream", err)
        }
    }
    
    @objc(initServices:deviceKey:deviceCert:seed:resolver:rejecter:)
    func initServices(_ apiKey:String, deviceKey:[UInt8], deviceCert:[UInt8], seed:[UInt8], resolver resolve: @escaping RCTPromiseResolveBlock, rejecter reject: @escaping RCTPromiseRejectBlock) -> Void {
        if self.breezServices != nil {
            reject(BreezSDK.TAG, "BreezServices already initialized", nil)
        } else {
            let greenlightCredentials = GreenlightCredentials(deviceKey: deviceKey, deviceCert: deviceCert)
            var config = breez_sdk.defaultConfig(envType: EnvironmentType.production)
            config.apiKey = apiKey
            
            do {
                self.breezServices = try breez_sdk.initServices(config: config, seed: seed, creds: greenlightCredentials, listener: BreezSDKListener(emitter: self))
                
                resolve(["status": "ok"])
            } catch let err {
                reject(BreezSDK.TAG, "Error calling initServices", err)
            }
        }
    }
    
    @objc(start:rejecter:)
    func start(_ resolve: @escaping RCTPromiseResolveBlock, rejecter reject: @escaping RCTPromiseRejectBlock) -> Void {
        if let breezServices = self.breezServices {
            do {
                try breezServices.start()
                
                resolve(["status": "ok"])
            } catch let err {
                reject(BreezSDK.TAG, "Error calling start", err)
            }
        } else {
            reject(BreezSDK.TAG, "BreezServices not initialized", nil)
        }
    }
    
    @objc(sync:rejecter:)
    func sync(_ resolve: @escaping RCTPromiseResolveBlock, rejecter reject: @escaping RCTPromiseRejectBlock) -> Void {
        if let breezServices = self.breezServices {
            do {
                try breezServices.sync()
                
                resolve(["status": "ok"])
            } catch let err {
                reject(BreezSDK.TAG, "Error calling sync", err)
            }
        } else {
            reject(BreezSDK.TAG, "BreezServices not initialized", nil)
        }
   }
    
    @objc(stop:rejecter:)
    func stop(_ resolve: @escaping RCTPromiseResolveBlock, rejecter reject: @escaping RCTPromiseRejectBlock) -> Void {
        if let breezServices = self.breezServices {
            do {
                try breezServices.stop()
                
                resolve(["status": "ok"])
            } catch let err {
                reject(BreezSDK.TAG, "Error calling stop", err)
            }
        } else {
            reject(BreezSDK.TAG, "BreezServices not initialized", nil)
        }
    }
    
    @objc(sendPayment:amountSats:resolver:rejecter:)
    func sendPayment(_ bolt11:String, amountSats:UInt64, resolver resolve: @escaping RCTPromiseResolveBlock, rejecter reject: @escaping RCTPromiseRejectBlock) -> Void {
        if let breezServices = self.breezServices {
            do {
                let optionalAmountSats = amountSats == 0 ? nil : amountSats
                let payment = try breezServices.sendPayment(bolt11: bolt11, amountSats: optionalAmountSats)
                
                resolve(BreezSDKMapper.dictionaryOf(payment: payment))
            } catch let err {
                reject(BreezSDK.TAG, "Error calling sendPayment", err)
            }
        } else {
            reject(BreezSDK.TAG, "BreezServices not initialized", nil)
        }
    }
    
    @objc(sendSpontaneousPayment:amountSats:resolver:rejecter:)
    func sendSpontaneousPayment(_ nodeId:String, amountSats:UInt64, resolver resolve: @escaping RCTPromiseResolveBlock, rejecter reject: @escaping RCTPromiseRejectBlock) -> Void {
        if let breezServices = self.breezServices {
            do {
                let payment = try breezServices.sendSpontaneousPayment(nodeId: nodeId, amountSats: amountSats)
                    
                resolve(BreezSDKMapper.dictionaryOf(payment: payment))
            } catch let err {
                reject(BreezSDK.TAG, "Error calling sendSpontaneousPayment", err)
            }
        } else {
            reject(BreezSDK.TAG, "BreezServices not initialized", nil)
        }
    }
    
    @objc(receivePayment:description:resolver:rejecter:)
    func receivePayment(_ amountSats:UInt64, description:String, resolver resolve: @escaping RCTPromiseResolveBlock, rejecter reject: @escaping RCTPromiseRejectBlock) -> Void {
        if let breezServices = self.breezServices {
            do {
                let lnInvoice = try breezServices.receivePayment(amountSats: amountSats, description: description)
                    
                resolve(BreezSDKMapper.dictionaryOf(lnInvoice: lnInvoice))
            } catch let err {
                reject(BreezSDK.TAG, "Error calling receivePayment", err)
            }
        } else {
            reject(BreezSDK.TAG, "BreezServices not initialized", nil)
        }
    }
    
    @objc(withdrawLnurl:amountSats:description:resolver:rejecter:)
    func withdrawLnurl(_ reqData:[String: Any], amountSats:UInt64, description:String?, resolver resolve: @escaping RCTPromiseResolveBlock, rejecter reject: @escaping RCTPromiseRejectBlock) -> Void {
        if let breezServices = self.breezServices {
            do {
                if let lnUrlWithdrawRequestData = BreezSDKMapper.asLnUrlWithdrawRequestData(reqData: reqData) {
                    let lnUrlWithdrawCallbackStatus = try breezServices.withdrawLnurl(reqData: lnUrlWithdrawRequestData, amountSats: amountSats, description: description)
                        
                    resolve(BreezSDKMapper.dictionaryOf(lnUrlWithdrawCallbackStatus: lnUrlWithdrawCallbackStatus))
                } else {
                    reject(BreezSDK.TAG, "Invalid reqData", nil)
                }
            } catch let err {
                reject(BreezSDK.TAG, "Error calling withdrawLnurl", err)
            }
        } else {
            reject(BreezSDK.TAG, "BreezServices not initialized", nil)
        }
    }
    
    @objc(nodeInfo:rejecter:)
    func nodeInfo(_ resolve: @escaping RCTPromiseResolveBlock, rejecter reject: @escaping RCTPromiseRejectBlock) -> Void {
        if let breezServices = self.breezServices {
            do {
                if let nodeState = try breezServices.nodeInfo() {
                    resolve(BreezSDKMapper.dictionaryOf(nodeState: nodeState))
                } else {
                    reject(BreezSDK.TAG, "No available node info", nil)
                }
            } catch let err {
                reject(BreezSDK.TAG, "Error calling nodeInfo", err)
            }
        } else {
            reject(BreezSDK.TAG, "BreezServices not initialized", nil)
        }
    }
    
    @objc(listPayments:fromTimestamp:toTimestamp:resolver:rejecter:)
    func listPayments(_ filter:String, fromTimestamp:Int64, toTimestamp:Int64, resolver resolve: @escaping RCTPromiseResolveBlock, rejecter reject: @escaping RCTPromiseRejectBlock) -> Void {
        if let breezServices = self.breezServices {
            do {
                let optionalFromTimestamp = fromTimestamp == 0 ? nil : fromTimestamp
                let optionalToTimestamp = toTimestamp == 0 ? nil : toTimestamp
                let payments = try breezServices.listPayments(filter: BreezSDKMapper.asPaymentTypeFilter(filter: filter), fromTimestamp: optionalFromTimestamp, toTimestamp: optionalToTimestamp)
                        
                resolve(BreezSDKMapper.arrayOf(payments: payments))
            } catch let err {
                reject(BreezSDK.TAG, "Error calling listPayments", err)
            }
        } else {
            reject(BreezSDK.TAG, "BreezServices not initialized", nil)
        }
    }
    
    @objc(sweep:feeRateSatsPerByte:resolver:rejecter:)
    func sweep(_ toAddress:String, feeRateSatsPerByte:UInt64, resolver resolve: @escaping RCTPromiseResolveBlock, rejecter reject: @escaping RCTPromiseRejectBlock) -> Void {
        if let breezServices = self.breezServices {
            do {
                try breezServices.sweep(toAddress: toAddress, feeRateSatsPerByte: feeRateSatsPerByte)
                    
                resolve(["status": "ok"])
            } catch let err {
                reject(BreezSDK.TAG, "Error calling sweep", err)
            }
        } else {
            reject(BreezSDK.TAG, "BreezServices not initialized", nil)
        }
    }
    
    @objc(fetchFiatRates:rejecter:)
    func fetchFiatRates(_ resolve: @escaping RCTPromiseResolveBlock, rejecter reject: @escaping RCTPromiseRejectBlock) -> Void {
        if let breezServices = self.breezServices {
            do {
                let rates = try breezServices.fetchFiatRates()
                        
                resolve(BreezSDKMapper.arrayOf(rates: rates))
            } catch let err {
                reject(BreezSDK.TAG, "Error calling fetchFiatRates", err)
            }
        } else {
            reject(BreezSDK.TAG, "BreezServices not initialized", nil)
        }
    }
    
    @objc(listFiatCurrencies:rejecter:)
    func listFiatCurrencies(_ resolve: @escaping RCTPromiseResolveBlock, rejecter reject: @escaping RCTPromiseRejectBlock) -> Void {
        if let breezServices = self.breezServices {
            do {
                let fiatCurrencies = try breezServices.listFiatCurrencies()
                        
                resolve(BreezSDKMapper.arrayOf(fiatCurrencies: fiatCurrencies))
            } catch let err {
                reject(BreezSDK.TAG, "Error calling listFiatCurrencies", err)
            }
        } else {
            reject(BreezSDK.TAG, "BreezServices not initialized", nil)
        }
    }
    
    @objc(listLsps:rejecter:)
    func listLsps(_ resolve: @escaping RCTPromiseResolveBlock, rejecter reject: @escaping RCTPromiseRejectBlock) -> Void {
        if let breezServices = self.breezServices {
            do {
                let lsps = try breezServices.listLsps()
                        
                resolve(BreezSDKMapper.arrayOf(lsps: lsps))
            } catch let err {
                reject(BreezSDK.TAG, "Error calling listLsps", err)
            }
        } else {
            reject(BreezSDK.TAG, "BreezServices not initialized", nil)
        }
    }
    
    @objc(connectLsp:resolver:rejecter:)
    func connectLsp(_ lspId:String, resolver resolve: @escaping RCTPromiseResolveBlock, rejecter reject: @escaping RCTPromiseRejectBlock) -> Void {
        if let breezServices = self.breezServices {
            do {
                try breezServices.connectLsp(lspId: lspId)
                    
                resolve(["status": "ok"])
            } catch let err {
                reject(BreezSDK.TAG, "Error calling connectLsp", err)
            }
        } else {
            reject(BreezSDK.TAG, "BreezServices not initialized", nil)
        }
    }
    
    @objc(fetchLspInfo:resolver:rejecter:)
    func fetchLspInfo(_ lspId:String, resolver resolve: @escaping RCTPromiseResolveBlock, rejecter reject: @escaping RCTPromiseRejectBlock) -> Void {
        if let breezServices = self.breezServices {
            do {
                if let lspInformation = try breezServices.fetchLspInfo(lspId: lspId) {
                    resolve(BreezSDKMapper.dictionaryOf(lspInformation: lspInformation))
                } else {
                    reject(BreezSDK.TAG, "No available lsp info", nil)
                }
            } catch let err {
                reject(BreezSDK.TAG, "Error calling fetchLspInfo", err)
            }
        } else {
            reject(BreezSDK.TAG, "BreezServices not initialized", nil)
        }
    }
    
    @objc(lspId:rejecter:)
    func lspId(_ resolve: @escaping RCTPromiseResolveBlock, rejecter reject: @escaping RCTPromiseRejectBlock) -> Void {
        if let breezServices = self.breezServices {
            do {
                if let lspId = try breezServices.lspId() {
                    resolve(lspId)
                } else {
                    reject(BreezSDK.TAG, "No available lsp id", nil)
                }
            } catch let err {
                reject(BreezSDK.TAG, "Error calling lspId", err)
            }
        } else {
            reject(BreezSDK.TAG, "BreezServices not initialized", nil)
        }
    }
    
    @objc(closeLspChannels:rejecter:)
    func closeLspChannels(_ resolve: @escaping RCTPromiseResolveBlock, rejecter reject: @escaping RCTPromiseRejectBlock) -> Void {
        if let breezServices = self.breezServices {
            do {
                try breezServices.closeLspChannels()
                    
                resolve(["status": "ok"])
            } catch let err {
                reject(BreezSDK.TAG, "Error calling closeLspChannels", err)
            }
        } else {
            reject(BreezSDK.TAG, "BreezServices not initialized", nil)
        }
    }
    
    @objc(receiveOnchain:rejecter:)
    func receiveOnchain(_ resolve: @escaping RCTPromiseResolveBlock, rejecter reject: @escaping RCTPromiseRejectBlock) -> Void {
        if let breezServices = self.breezServices {
            do {
                let swapInfo = try breezServices.receiveOnchain()
                        
                resolve(BreezSDKMapper.dictionaryOf(swapInfo: swapInfo))
            } catch let err {
                reject(BreezSDK.TAG, "Error calling receiveOnchain", err)
            }
        } else {
            reject(BreezSDK.TAG, "BreezServices not initialized", nil)
        }
    }
    
    @objc(inProgressSwap:rejecter:)
    func inProgressSwap(_ resolve: @escaping RCTPromiseResolveBlock, rejecter reject: @escaping RCTPromiseRejectBlock) -> Void {
        if let breezServices = self.breezServices {
            do {
                if let swapInfo = try breezServices.inProgressSwap() {
                    resolve(BreezSDKMapper.dictionaryOf(swapInfo: swapInfo))
                } else {
                    reject(BreezSDK.TAG, "No available in progress swap", nil)
                }
            } catch let err {
                reject(BreezSDK.TAG, "Error calling inProgressSwap", err)
            }
        } else {
            reject(BreezSDK.TAG, "BreezServices not initialized", nil)
        }
    }
    
    @objc(listRefundables:rejecter:)
    func listRefundables(_ resolve: @escaping RCTPromiseResolveBlock, rejecter reject: @escaping RCTPromiseRejectBlock) -> Void {
        if let breezServices = self.breezServices {
            do {
                let swapInfos = try breezServices.listRefundables()
                        
                resolve(BreezSDKMapper.arrayOf(swapInfos: swapInfos))
            } catch let err {
                reject(BreezSDK.TAG, "Error calling listRefundables", err)
            }
        } else {
            reject(BreezSDK.TAG, "BreezServices not initialized", nil)
        }
    }
    
    @objc(refund:fromTimestamp:toTimestamp:resolver:rejecter:)
    func refund(_ swapAddress:String, toAddress:String, satPerVbyte:UInt32, resolver resolve: @escaping RCTPromiseResolveBlock, rejecter reject: @escaping RCTPromiseRejectBlock) -> Void {
        if let breezServices = self.breezServices {
            do {
                let result = try breezServices.refund(swapAddress: swapAddress, toAddress: toAddress, satPerVbyte: satPerVbyte)
                
                resolve(result)
            } catch let err {
                reject(BreezSDK.TAG, "Error calling refund", err)
            }
        } else {
            reject(BreezSDK.TAG, "BreezServices not initialized", nil)
        }
    }
    
    @objc(executeDevCommand:resolver:rejecter:)
    func executeDevCommand(_ command:String, resolver resolve: @escaping RCTPromiseResolveBlock, rejecter reject: @escaping RCTPromiseRejectBlock) -> Void {
        if let breezServices = self.breezServices {
            do {
                let result = try breezServices.executeDevCommand(command: command)
                
                resolve(result)
            } catch let err {
                reject(BreezSDK.TAG, "Error calling executeDevCommand", err)
            }
        } else {
            reject(BreezSDK.TAG, "BreezServices not initialized", nil)
        }
    }
    
    @objc(recommendedFees:rejecter:)
    func recommendedFees(_ resolve: @escaping RCTPromiseResolveBlock, rejecter reject: @escaping RCTPromiseRejectBlock) -> Void {
        if let breezServices = self.breezServices {
            do {
                let fees = try breezServices.recommendedFees()
                        
                resolve(BreezSDKMapper.dictionaryOf(recommendedFees: fees))
            } catch let err {
                reject(BreezSDK.TAG, "Error calling recommendedFees", err)
            }
        } else {
            reject(BreezSDK.TAG, "BreezServices not initialized", nil)
        }
    }
}
