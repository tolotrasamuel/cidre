//
//  vn.h
//  vn
//
//  Created by Yury Korolev on 13.10.2022.
//


#import <Foundation/Foundation.h>
#import <Vision/Vision.h>
#import "../macro.h"

NS_ASSUME_NONNULL_BEGIN

#pragma mark - VNRequest

rwsel(, id, revision, setRevision, NSUInteger)
rwsel(, id, usesCPUOnly, setUsesCPUOnly, BOOL)

#pragma mark - VNImageBasedRequest

//@property (readwrite, nonatomic, assign) CGRect regionOfInterest;
rwsel(, id, regionOfInterest, setRegionOfInterest, CGRect)

#pragma mark - VNImageRequestHandler

NS_RETURNS_RETAINED
cinit_ab(, VNImageRequestHandler, initWithURL, NSURL *, options, NSDictionary *);

NS_RETURNS_RETAINED
cinit_abc(, VNImageRequestHandler, initWithURL, NSURL *, orientation, CGImagePropertyOrientation, options, NSDictionary *);

NS_RETURNS_RETAINED
cinit_ab(, VNImageRequestHandler, initWithCVPixelBuffer, CVPixelBufferRef, options, NSDictionary *);


NS_RETURNS_RETAINED
cinit_abc(, VNImageRequestHandler, initWithCVPixelBuffer, CVPixelBufferRef, orientation, CGImagePropertyOrientation, options, NSDictionary *);

rsel_ab(, id, performRequests, NSArray<VNRequest *> *, error, NSError **, BOOL)

#pragma mark - VNSequenceRequestHandler

csel(, VNSequenceRequestHandler, new, VNSequenceRequestHandler *)

rsel_abc(, id, performRequests, NSArray<VNRequest *> *, onCVPixelBuffer, CVPixelBufferRef, error, NSError **, BOOL)

rsel_abc(, id, performRequests, NSArray<VNRequest *> *, onCMSampleBuffer, CMSampleBufferRef, error, NSError **, BOOL)

#pragma mark - VNObservation

rsel(, id, uuid, NSUUID *)
rsel(, id, confidence, VNConfidence)
rsel(, id, timeRange, CMTimeRange)


#pragma mark - VNDetectedObjectObservation

//@property (readonly, nonatomic, assign) CGRect boundingBox;
rsel(, id, boundingBox, CGRect)

rsel(, id, globalSegmentationMask, VNPixelBufferObservation *)

#pragma mark -VNHorizon

NS_RETURNS_RETAINED
csel(, VNDetectHorizonRequest, new, VNDetectHorizonRequest *)


#pragma mark - VNHorizonObservation



//@property (readonly, nonatomic, assign) CGAffineTransform transform;
rsel(, id, transform, CGAffineTransform)

rsel(, id, angle, CGFloat)
//@property (readonly, nonatomic, assign) CGFloat angle;

rsel_ab(, id, transformForImageWidth, size_t, height, size_t, CGAffineTransform)

#pragma mark - VNFaceObservation

rsel(, id, landmarks, VNFaceLandmarks2D *)
rsel(, id, faceCaptureQuality, NSNumber *)

rsel(, id, roll, NSNumber *)
rsel(, id, yaw, NSNumber *)
rsel(, id, pitch, NSNumber *)

#pragma mark - VNClassificationObservation

rsel(, id, identifier, NSString *)

rsel(, id, hasPrecisionRecallCurve, BOOL)


//- (BOOL) hasMinimumRecall:(float)minimumRecall forPrecision:(float)precision;
rsel_ab(, id, hasMinimumRecall, float, forPrecision, float, BOOL)
//- (BOOL) hasMinimumPrecision:(float)minimumPrecision forRecall:(float)recall;
rsel_ab(, id, hasMinimumPrecision, float, forRecall, float, BOOL)

#pragma mark - VNHumanObservation

rsel(, id, upperBodyOnly, BOOL)

#pragma mark - VNRecognizedObjectObservation

rsel(, id, labels, NSArray *)

#pragma mark - VNPixelBufferObservation

CF_RETURNS_NOT_RETAINED
rsel(, id, pixelBuffer, CVPixelBufferRef)

rsel(, id, featureName, NSString *)

#pragma mark - VNTextObservation

rsel(, id, characterBoxes, NSArray *)// NSArray<VNRectangleObservation *> *characterBoxes;


#pragma mark - VNGeneratePersonSegmentationRequest

rwsel(, id, qualityLevel, setQualityLevel, VNGeneratePersonSegmentationRequestQualityLevel)

//@property (readwrite, nonatomic) OSType outputPixelFormat;
rwsel(, id, outputPixelFormat, setOutputPixelFormat, OSType)

NS_RETURNS_RETAINED
csel(, VNGeneratePersonSegmentationRequest, new, VNGeneratePersonSegmentationRequest *)


#pragma mark - VNDetectDocumentSegmentationRequest

NS_RETURNS_RETAINED
csel(, VNDetectDocumentSegmentationRequest, new, VNDetectDocumentSegmentationRequest *)

#pragma mark - VNSaliencyImageObservation

rsel(, id, salientObjects, NSArray *)

#pragma mark - VNGenerateAttentionBasedSaliencyImageRequest

NS_RETURNS_RETAINED
csel(, VNGenerateAttentionBasedSaliencyImageRequest, new, VNGenerateAttentionBasedSaliencyImageRequest *)

#pragma mark - VNGenerateObjectnessBasedSaliencyImageRequest

NS_RETURNS_RETAINED
csel(, VNGenerateObjectnessBasedSaliencyImageRequest, new, VNGenerateObjectnessBasedSaliencyImageRequest *)


#pragma mark - VNFaceLandmarkRegion

rsel(, id, pointCount, NSUInteger)

#pragma mark - VNClassifyImageRequest

//rsel_a(, id, supportedIdentifiersAndReturnError, NSError **, NSArray *)

NS_RETURNS_RETAINED
csel(, VNClassifyImageRequest, new, VNClassifyImageRequest *)

#pragma mark - VNRecognizeAnimalsRequest

NS_RETURNS_RETAINED
csel(, VNRecognizeAnimalsRequest, new, VNRecognizeAnimalsRequest *)

#pragma mark - VNFeaturePrintObservation

rsel(vn_, VNFeaturePrintObservation *, elementType, VNElementType)
rsel(vn_, VNFeaturePrintObservation *, elementCount, NSUInteger)
rsel(vn_, VNFeaturePrintObservation *, data, NSData *)

rsel_abc(, id, computeDistance, float *, toFeaturePrintObservation, VNFeaturePrintObservation *, error, NSError **, BOOL)

#pragma mark - VNGenerateImageFeaturePrintRequest

rwsel(, id, imageCropAndScaleOption, setImageCropAndScaleOption, VNImageCropAndScaleOption)

NS_RETURNS_RETAINED
csel(, VNGenerateImageFeaturePrintRequest, new, VNGenerateImageFeaturePrintRequest *)

#pragma mark - VNDetectFaceRectanglesRequest

NS_RETURNS_RETAINED
csel(, VNDetectFaceRectanglesRequest, new, VNDetectFaceRectanglesRequest *)

#pragma mark - VNTrackingRequest

rwsel(, id, trackingLevel, setTrackingLevel, VNRequestTrackingLevel)

#pragma mark - VNDetectBarcodesRequest
NS_RETURNS_RETAINED
csel(, VNDetectBarcodesRequest, new, VNDetectBarcodesRequest *)

//@property (readwrite, nonatomic, copy) NSArray<VNBarcodeSymbology> *symbologies;
rwsel(, id, symbologies, setSymbologies, NSArray *)

rsel_a(, id, supportedSymbologiesAndReturnError, NSError **, NSArray *)

#pragma mark - VNRecognizeTextRequest

csel(, VNRecognizeTextRequest, new, VNRecognizeTextRequest *)

SEL sel_results;
SEL sel_supportedIdentifiersAndReturnError;

__attribute__((constructor))
static void common_initializer()
{
  static int initialized = 0;
  if (!initialized) {
    sel_results = @selector(results);
    sel_supportedIdentifiersAndReturnError = @selector(supportedIdentifiersAndReturnError:);
  }
}



NS_ASSUME_NONNULL_END

